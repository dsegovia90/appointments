#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::{
    models::{
        admin_settings::AdminSettings,
        google_calendars::{self, GoogleCalendars},
        users::users,
    },
    views::google_calendars::{CalendarEntry, CalendarSettingParams, CalendarSettingsResponse},
};
use axum::response::Redirect;
use google_calendar::types::MinAccessRole;
use loco_rs::prelude::*;
use serde::Deserialize;

#[debug_handler]
pub async fn oauth_url(State(ctx): State<AppContext>, user: users::Model) -> Result<Json<String>> {
    // Return the URL using the struct
    let oauth_url = google_calendars::Model::generate_oauth_url(&ctx, &user).await?;
    Ok(Json(oauth_url.to_string()))
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQueryParams {
    pub code: String,
    pub state: Uuid,
    pub scope: String,
}

#[debug_handler]
pub async fn oauth_callback(
    State(ctx): State<AppContext>,
    Query(params): Query<OAuthCallbackQueryParams>,
) -> Result<Redirect> {
    // Exchange the authorization code for an access token
    let redirect_url = google_calendars::Model::exchange_code_for_token(&ctx, params).await?;

    Ok(Redirect::to(&redirect_url))
}

#[debug_handler]
pub async fn get_calendars(
    State(ctx): State<AppContext>,
    user: users::Model,
) -> Result<Json<Vec<CalendarEntry>>> {
    // Exchange the authorization code for an access token
    let google_calendar_settings = AdminSettings::get_google_calendar_settings(&ctx.db).await?;
    let google_calendar_config = GoogleCalendars::find_by_user(&ctx.db, &user).await?;
    let redirect_url = google_calendars::OAuthUrl::redirect_uri(&google_calendar_settings)?;

    let client = google_calendar::Client::new(
        google_calendar_settings.google_oauth_client_id,
        google_calendar_settings.google_oauth_secret,
        redirect_url,
        google_calendar_config.access_token,
        google_calendar_config.refresh_token,
    );

    let _access_token = client.refresh_access_token().await.map_err(Error::wrap)?;

    let calendar_list = client
        .calendar_list()
        .list_all(MinAccessRole::Reader, false, false)
        .await
        .map_err(Error::wrap)?;

    let calendar_entries: Vec<CalendarEntry> = calendar_list
        .body
        .into_iter()
        .map(CalendarEntry::from)
        .collect();

    Ok(Json(calendar_entries))
}

#[debug_handler]
pub async fn add_calendar(
    State(ctx): State<AppContext>,
    user: users::Model,
    Json(params): Json<CalendarSettingParams>,
) -> Result<Json<CalendarSettingsResponse>> {
    // Implementation goes here
    let google_calendar_config = GoogleCalendars::find_by_user(&ctx.db, &user).await?;

    let updated_google_calendar_config = google_calendar_config
        .add_calendar_to_settings(&ctx.db, params)
        .await?;

    Ok(Json(updated_google_calendar_config.into()))
}

#[debug_handler]
pub async fn remove_calendar(
    State(ctx): State<AppContext>,
    user: users::Model,
    Json(params): Json<CalendarSettingParams>,
) -> Result<Json<CalendarSettingsResponse>> {
    // Implementation goes here
    let google_calendar_config = GoogleCalendars::find_by_user(&ctx.db, &user).await?;

    let updated_google_calendar_config = google_calendar_config
        .remove_calendar_from_settings(&ctx.db, params)
        .await?;

    Ok(Json(updated_google_calendar_config.into()))
}

#[debug_handler]
pub async fn get_settings(
    State(ctx): State<AppContext>,
    user: users::Model,
) -> Result<Json<CalendarSettingsResponse>> {
    // Implementation goes here
    let google_calendar_config = GoogleCalendars::find_by_user(&ctx.db, &user).await?;

    Ok(Json(google_calendar_config.into()))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/google_calendar/")
        .add("/oauth_url", get(oauth_url))
        .add("/oauth_callback", get(oauth_callback))
        .add("/get_calendars", get(get_calendars))
        .add("/add_calendar", post(add_calendar))
        .add("/remove_calendar", post(remove_calendar))
        .add("/get_settings", get(get_settings))
}
