pub use super::_entities::google_calendars::{ActiveModel, Entity, Model};
use crate::{
    controllers::api::integrations::google_calendar::OAuthCallbackQueryParams,
    models::{
        _entities::{admin_settings::GoogleCalendarSettings, google_calendars::Column},
        admin_settings::AdminSettings,
        oauth_states::{self, OAuthStates},
        users::{self, Users},
    },
    views::{
        client_facing::AvailabilityWindow,
        google_calendars::{CalendarSettingParams, CalendarSettingType},
    },
};
use google_calendar::types::FreeBusyRequestItem;
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub type GoogleCalendars = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OAuthUrl(pub url::Url);

impl std::fmt::Display for OAuthUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl OAuthUrl {
    const REDIRECT_URI_PATH: &'static str = "/api/google_calendar/oauth_callback";
    const SCOPE: &'static str = "https://www.googleapis.com/auth/calendar.readonly https://www.googleapis.com/auth/calendar.events";

    pub fn new(
        csrf_token: Uuid,
        google_calendar_settings: &GoogleCalendarSettings,
    ) -> Result<Self> {
        let mut oauth_url =
            url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").map_err(Error::wrap)?;
        let redirect_uri = Self::redirect_uri(google_calendar_settings)?;

        oauth_url
            .query_pairs_mut()
            .append_pair(
                "client_id",
                &google_calendar_settings.google_oauth_client_id,
            )
            .append_pair("redirect_uri", redirect_uri.as_str())
            .append_pair("response_type", "code")
            .append_pair("scope", Self::SCOPE)
            .append_pair("access_type", "offline")
            .append_pair("state", &csrf_token.to_string())
            .append_pair("include_granted_scopes", "true")
            .append_pair("enable_granular_consent", "false")
            .append_pair("prompt", "consent");

        Ok(Self(oauth_url))
    }

    pub fn redirect_uri(google_calendar_settings: &GoogleCalendarSettings) -> Result<url::Url> {
        let mut redirect_uri = google_calendar_settings
            .google_oauth_redirect_uri_base
            .clone();
        redirect_uri.set_path(Self::REDIRECT_URI_PATH);
        Ok(redirect_uri)
    }
}

#[derive(Debug, Serialize)]
pub struct OAuthTokenRequest {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub grant_type: String,
}

impl OAuthTokenRequest {
    pub async fn new(ctx: &AppContext, code: String) -> Result<Self> {
        let google_calendar_settings = AdminSettings::get_google_calendar_settings(&ctx.db).await?;
        let redirect_uri = OAuthUrl::redirect_uri(&google_calendar_settings)?.to_string();

        Ok(Self {
            code,
            client_id: google_calendar_settings.google_oauth_client_id,
            client_secret: google_calendar_settings.google_oauth_secret,
            redirect_uri,
            grant_type: "authorization_code".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct OAuthTokenResponseSuccess {
    /// The token that your application sends to authorize a Google API request.
    pub access_token: String,
    /// The remaining lifetime of the access token in seconds.
    pub expires_in: i32,
    /// A token that you can use to obtain a new access token.
    /// Refresh tokens are valid until the user revokes access or the refresh token expires.
    pub refresh_token: String,
    /// The remaining lifetime of the refresh token in seconds. This value is only set when the user grants time-based access.
    pub refresh_token_expires_in: i32,
    /// The scopes of access granted by the `access_token` expressed as a list of space-delimited, case-sensitive strings.
    pub scope: String,
    /// The type of token returned. This value is always Bearer.
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthTokenResponseError {
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OAuthTokenResponse {
    Ok(OAuthTokenResponseSuccess),
    Err(OAuthTokenResponseError),
}

// implement your read-oriented logic here
impl Model {
    pub async fn generate_oauth_url(ctx: &AppContext, user: &users::Model) -> Result<OAuthUrl> {
        // Generate a CSRF token
        let oauth_state = oauth_states::ActiveModel::create(&ctx.db, user).await?;
        let google_calendar_settings = AdminSettings::get_google_calendar_settings(&ctx.db).await?;
        OAuthUrl::new(oauth_state.id, &google_calendar_settings)
    }

    pub async fn exchange_code_for_token(
        ctx: &AppContext,
        query_params: OAuthCallbackQueryParams,
    ) -> Result<String> {
        let token_request = OAuthTokenRequest::new(ctx, query_params.code).await?;
        let response = reqwest::Client::new()
            .post("https://oauth2.googleapis.com/token")
            .json(&token_request)
            .send()
            .await
            .map_err(Error::wrap)?
            .json::<OAuthTokenResponse>()
            .await
            .map_err(Error::wrap)?;

        match response {
            OAuthTokenResponse::Ok(auth_success_response) => {
                // TODO: Store the access_token and refresh_token in your database
                // associated with the user identified by params.state
                //
                let oauth_state =
                    OAuthStates::find_by_uuid_and_destroy(&ctx.db, &query_params.state)
                        .await
                        .map_err(|_| {
                            Error::Unauthorized(
                                "CSRF token not found. May be a CSRF attack".to_string(),
                            )
                        })?;
                let user = Users::find_by_id(&ctx.db, oauth_state.user_id).await?;
                ActiveModel::create(&ctx.db, auth_success_response, user).await?;

                Ok("/dashboard/integrations?success".to_string())
            }
            OAuthTokenResponse::Err(auth_token_err_response) => {
                tracing::error!("Google OAuth error: {:?}", auth_token_err_response);

                Ok(format!(
                    "/dashboard/integrations?error={}&description={}",
                    auth_token_err_response.error, auth_token_err_response.error_description
                ))
            }
        }
    }

    pub async fn add_calendar_to_settings<C: ConnectionTrait>(
        &self,
        db: &C,
        params: CalendarSettingParams,
    ) -> Result<Self> {
        let mut active_model = self.clone().into_active_model();
        match params.setting_type {
            CalendarSettingType::CollisionCheck => {
                let mut cloned = self.calendars_for_collision_check.clone();
                cloned.add(params.calendar_id);
                active_model.calendars_for_collision_check = ActiveValue::Set(cloned);
            }
            CalendarSettingType::EventHandling => {
                let mut cloned = self.calendars_for_event_handling.clone();
                cloned.add(params.calendar_id);
                active_model.calendars_for_event_handling = ActiveValue::Set(cloned);
            }
        }

        Ok(active_model.update(db).await?)
    }

    pub async fn remove_calendar_from_settings<C: ConnectionTrait>(
        &self,
        db: &C,
        params: CalendarSettingParams,
    ) -> Result<Self> {
        let mut active_model = self.clone().into_active_model();
        match params.setting_type {
            CalendarSettingType::CollisionCheck => {
                let mut cloned = self.calendars_for_collision_check.clone();
                cloned.remove(&params.calendar_id);
                active_model.calendars_for_collision_check = ActiveValue::Set(cloned);
            }
            CalendarSettingType::EventHandling => {
                let mut cloned = self.calendars_for_event_handling.clone();
                cloned.remove(&params.calendar_id);
                active_model.calendars_for_event_handling = ActiveValue::Set(cloned);
            }
        }

        Ok(active_model.update(db).await?)
    }

    pub async fn get_free_busy<C: ConnectionTrait>(
        db: &C,
        user: &users::Model,
        time_min: DateTimeUtc,
        time_max: DateTimeUtc,
    ) -> Result<Vec<AvailabilityWindow>> {
        let google_calendar_settings = AdminSettings::get_google_calendar_settings(db).await?;
        let google_calendar_config = GoogleCalendars::find_by_user(db, user).await?;
        let redirect_url = OAuthUrl::redirect_uri(&google_calendar_settings)?;

        let client = google_calendar::Client::new(
            google_calendar_settings.google_oauth_client_id,
            google_calendar_settings.google_oauth_secret,
            redirect_url,
            google_calendar_config.access_token,
            google_calendar_config.refresh_token,
        );

        let _access_token = client.refresh_access_token().await.map_err(Error::wrap)?;

        let items = google_calendar_config
            .calendars_for_collision_check
            .0
            .into_iter()
            .map(|id| FreeBusyRequestItem { id })
            .collect();

        let query = google_calendar::types::FreeBusyRequest {
            items,
            time_min: Some(time_min),
            time_max: Some(time_max),
            calendar_expansion_max: 0,
            group_expansion_max: 0,
            time_zone: String::new(),
        };

        let free_busy_response = client.freebusy().query(&query).await.map_err(Error::wrap)?;
        let availability_windows = free_busy_response
            .body
            .calendars
            .into_iter()
            .flat_map(|cal| {
                cal.1.busy.into_iter().map(|x| AvailabilityWindow {
                    start: x.start,
                    end: x.end,
                })
            })
            .collect();
        Ok(availability_windows)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create<C: ConnectionTrait>(
        db: &C,
        props: OAuthTokenResponseSuccess,
        user: users::Model,
    ) -> Result<Model> {
        if let Ok(existing_google_calendar) = GoogleCalendars::find_by_user(db, &user).await {
            existing_google_calendar
                .into_active_model()
                .delete(db)
                .await?;
        }

        let active_model = Self {
            access_token: ActiveValue::Set(props.access_token),
            expires_in: ActiveValue::Set(props.expires_in),
            refresh_token: ActiveValue::Set(props.refresh_token),
            refresh_token_expires_in: ActiveValue::Set(props.refresh_token_expires_in),
            scope: ActiveValue::Set(props.scope),
            token_type: ActiveValue::Set(props.token_type),
            user_id: ActiveValue::Set(user.id),
            ..Default::default()
        };
        Ok(active_model.insert(db).await?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_user<C: ConnectionTrait>(db: &C, user: &users::Model) -> Result<Model> {
        let query = Self::find().filter(Column::UserId.eq(user.id));
        query
            .one(db)
            .await?
            .ok_or(Error::Model(ModelError::EntityNotFound))
    }
}
