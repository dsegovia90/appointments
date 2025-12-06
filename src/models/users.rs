use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use chrono::{offset::Local, Duration, DurationRound, TimeDelta};
use chrono_tz::{ParseError, Tz};
use loco_rs::{auth::jwt, hash, prelude::*};
use now::DateTimeNow;
use sea_orm::PaginatorTrait;
use serde::Deserialize;
use serde_json::Map;
use uuid::Uuid;
use validator::ValidationError;
pub type Users = Entity;

use crate::{
    models::{
        admin_settings::AdminSettings,
        appointment_types, appointments, google_calendars,
        users::users::Role,
        weekly_availabilities::{self, WeeklyAvailabilityDuration},
    },
    our_chrono,
    traits::GenericWindowComparison,
    views::client_facing::AvailabilityWindow,
};

use super::_entities;
pub use super::_entities::users::{self, ActiveModel, Entity, Model};

pub const MAGIC_LINK_LENGTH: i8 = 32;
pub const MAGIC_LINK_EXPIRATION_MIN: i8 = 5;

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub name: String,
}

fn is_valid_timezone(value: &str) -> Result<(), ValidationError> {
    let _tz: Tz = value.parse().map_err(|_e: ParseError| {
        ValidationError::new("timezone").with_message("Invalid timezone".into())
    })?;
    Ok(())
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(email(message = "invalid email"))]
    pub email: String,
    #[validate(custom(function = "is_valid_timezone"))]
    pub timezone: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.name.as_ref().to_owned(),
            email: self.email.as_ref().to_owned(),
            timezone: self.timezone.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::users::ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if insert {
            self.pid = ActiveValue::Set(Uuid::new_v4());
            self.api_key = ActiveValue::Set(format!("ap-{}", Uuid::new_v4()));

            let user_count = Users::find().count(db).await?;
            if user_count == 0 {
                self.role = ActiveValue::Set(Role::Admin);
            }
        }
        Ok(self)
    }
}

#[async_trait]
impl Authenticable for Model {
    async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_pid(db, claims_key).await
    }
}

#[derive(Debug, Validate)]
#[validate(schema(function = "validate_current_availability_props"))]
pub struct CurrentAvailabilityProps<'a> {
    pub appointment_type: &'a appointment_types::Model,
    pub start_how_far_from_now: Duration,
    pub end_how_far_from_now: Duration,
}

fn validate_current_availability_props(
    value: &CurrentAvailabilityProps<'_>,
) -> Result<(), ValidationError> {
    if value.start_how_far_from_now >= value.end_how_far_from_now {
        return Err(ValidationError::new(
            "\"start_how_far_from_now\" must be less than \"end_how_far_from_now\".",
        ));
    }
    Ok(())
}

impl Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, email)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::EmailVerificationToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the magic token and verify and token expiration
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error ot token expired
    pub async fn find_by_magic_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                query::condition()
                    .eq(users::Column::MagicLinkToken, token)
                    .build(),
            )
            .one(db)
            .await?;

        let user = user.ok_or_else(|| ModelError::EntityNotFound)?;
        if let Some(expired_at) = user.magic_link_expiration {
            if expired_at >= Local::now() {
                Ok(user)
            } else {
                tracing::debug!(
                    user_pid = user.pid.to_string(),
                    token_expiration = expired_at.to_string(),
                    "magic token expired for the user."
                );
                Err(ModelError::msg("magic token expired"))
            }
        } else {
            tracing::error!(
                user_pid = user.pid.to_string(),
                "magic link expiration time not exists"
            );
            Err(ModelError::msg("expiration token not exists"))
        }
    }

    /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ResetToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided api key
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn create_with_password(
        db: &DatabaseConnection,
        params: &RegisterParams,
        timezone: Tz,
    ) -> ModelResult<Self> {
        let admin_settings = AdminSettings::find()
            .one(db)
            .await?
            .ok_or(ModelError::EntityNotFound)?;
        if !admin_settings.allow_new_registrations {
            return Err(ModelError::Message(
                "Admin Settings preveting user registration.".to_string(),
            ));
        }

        let txn = db.begin().await?;

        if users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, &params.email)
                    .build(),
            )
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists {});
        }

        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let user = users::ActiveModel {
            email: ActiveValue::set(params.email.clone()),
            password: ActiveValue::set(password_hash),
            name: ActiveValue::set(params.name.clone()),
            timezone: ActiveValue::set(timezone.to_string()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(user)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: u64) -> ModelResult<String> {
        jwt::JWT::new(secret)
            .generate_token(expiration, self.pid.to_string(), Map::new())
            .map_err(ModelError::from)
    }

    pub async fn get_current_availabilities_by_appointment_type<C>(
        &self,
        db: &C,
        props: CurrentAvailabilityProps<'_>,
    ) -> ModelResult<Vec<AvailabilityWindow>>
    where
        C: ConnectionTrait,
    {
        validator::Validate::validate(&props).map_err(ModelError::wrap)?;
        let appointments = appointments::Appointments::find_upcoming(db, self).await?;

        let google_calendar_windows = match google_calendars::Model::get_free_busy(
            db,
            self,
            chrono::Utc::now() + props.start_how_far_from_now,
            chrono::Utc::now() + props.end_how_far_from_now,
        )
        .await
        {
            Ok(windows) => windows,
            Err(err) => {
                tracing::warn!("Error getting free/busy: {}", err.to_string());
                Vec::new()
            }
        };

        let mut my_vec: Vec<AvailabilityWindow> = Vec::new();
        my_vec.extend(google_calendar_windows);
        my_vec.extend(appointments.into_iter().map(|a| AvailabilityWindow {
            start: a.start_time.to_utc(),
            end: a.endtime.to_utc(),
        }));

        let user_timezone: Tz = self.timezone.parse().map_err(ModelError::wrap)?;
        let weekly_availabilities: Vec<WeeklyAvailabilityDuration> =
            weekly_availabilities::Entity::find_by_user(db, self, vec![])
                .await?
                .into_iter()
                .map(Into::into)
                .collect();
        let appointment_duration =
            Duration::minutes(i64::from(props.appointment_type.duration_in_minutes));

        let mut time_cursor = (our_chrono::utc_now().with_timezone(&user_timezone)
            + props.start_how_far_from_now)
            .duration_round(TimeDelta::minutes(1))
            .map_err(ModelError::wrap)?;

        let up_until = (our_chrono::utc_now().with_timezone(&user_timezone)
            + props.end_how_far_from_now)
            .duration_round(TimeDelta::minutes(1))
            .map_err(ModelError::wrap)?;

        let mut windows: Vec<AvailabilityWindow> = Vec::new();

        while time_cursor < up_until {
            let monday_at_00 = time_cursor.beginning_of_week();

            let duration_between_time_cursor_and_monday_at_00 = (monday_at_00 - time_cursor).abs();

            let avail = weekly_availabilities
                .iter()
                .find(|item| item.to > duration_between_time_cursor_and_monday_at_00);

            match avail {
                // Availability found ahead of current time_cursor, but still
                // outside the window, move time_cursor to found "from".
                Some(avail) if avail.from > duration_between_time_cursor_and_monday_at_00 => {
                    time_cursor =
                        time_cursor + avail.from - duration_between_time_cursor_and_monday_at_00;
                }
                Some(avail)
                    if (duration_between_time_cursor_and_monday_at_00 - avail.from)
                        .num_minutes()
                        % appointment_duration.num_minutes()
                        != 0 =>
                {
                    let remainder = (duration_between_time_cursor_and_monday_at_00 - avail.from)
                        .num_minutes()
                        % appointment_duration.num_minutes();

                    time_cursor = time_cursor + appointment_duration - Duration::minutes(remainder);
                }
                // Availability found this week for current time_cursor, and time_cursor is
                // inside the window, use the duration from "from" to "to".
                Some(avail) => {
                    if duration_between_time_cursor_and_monday_at_00 >= avail.from
                        && duration_between_time_cursor_and_monday_at_00 + appointment_duration
                            <= avail.to
                    {
                        let window = AvailabilityWindow {
                            start: time_cursor.to_utc(),
                            end: (time_cursor + appointment_duration).to_utc(),
                        };
                        if !window.clash_check(&my_vec) {
                            windows.push(window);
                        }
                    }
                    time_cursor += appointment_duration;
                }
                // No availability found this week for current time_cursor,
                // jump to next monday time 00:00
                None => {
                    time_cursor = time_cursor
                        .end_of_week()
                        .duration_round(TimeDelta::minutes(1))
                        .map_err(ModelError::wrap)?;
                }
            }
        }

        Ok(windows)
    }

    #[must_use]
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}

impl ActiveModel {
    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.email_verification_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.reset_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.reset_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.email_verified_at = ActiveValue::set(Some(Local::now().into()));
        Ok(self.update(db).await?)
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<Model> {
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        self.reset_token = ActiveValue::Set(None);
        self.reset_sent_at = ActiveValue::Set(None);
        Ok(self.update(db).await?)
    }

    /// Creates a magic link token for passwordless authentication.
    ///
    /// Generates a random token with a specified length and sets an expiration time
    /// for the magic link. This method is used to initiate the magic link authentication flow.
    ///
    /// # Errors
    /// - Returns an error if database update fails
    pub async fn create_magic_link(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        let random_str = hash::random_string(MAGIC_LINK_LENGTH as usize);
        let expired = Local::now() + Duration::minutes(MAGIC_LINK_EXPIRATION_MIN.into());

        self.magic_link_token = ActiveValue::set(Some(random_str));
        self.magic_link_expiration = ActiveValue::set(Some(expired.into()));
        Ok(self.update(db).await?)
    }

    /// Verifies and invalidates the magic link after successful authentication.
    ///
    /// Clears the magic link token and expiration time after the user has
    /// successfully authenticated using the magic link.
    ///
    /// # Errors
    /// - Returns an error if database update fails
    pub async fn clear_magic_link(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.magic_link_token = ActiveValue::set(None);
        self.magic_link_expiration = ActiveValue::set(None);
        Ok(self.update(db).await?)
    }
}

impl Entity {
    pub async fn find_by_id<C>(db: &C, id: i32) -> ModelResult<Model>
    where
        C: ConnectionTrait,
    {
        let user = Self::find()
            .filter(_entities::users::Column::Id.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        Ok(user)
    }
}

impl<S> FromRequestParts<S> for users::Model
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_jwt = auth::JWTWithUser::<Self>::from_request_parts(parts, state).await?;

        Ok(auth_jwt.user)
    }
}

impl<S> OptionalFromRequestParts<S> for users::Model
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let auth_jwt = auth::JWTWithUser::<Self>::from_request_parts(parts, state).await;

        Ok(auth_jwt.ok().map(|auth_jwt| auth_jwt.user))
    }
}
