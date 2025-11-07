#![allow(non_upper_case_globals)]

use std::str::FromStr;

use chrono_tz::Tz;
use loco_rs::prelude::*;
use serde_json::json;

use crate::models::{appointments, users::Users};

static notify_user: Dir<'_> = include_dir!("src/mailers/appointments/notify_user");
static notify_client: Dir<'_> = include_dir!("src/mailers/appointments/notify_client");

#[allow(clippy::module_name_repetitions)]
pub struct AppointmentsMailer {}
impl Mailer for AppointmentsMailer {
    fn opts() -> mailer::MailerOpts {
        mailer::MailerOpts {
            from: std::env::var("DEFAULT_EMAIL_SENDER").unwrap(),
            ..Default::default()
        }
    }
}
impl AppointmentsMailer {
    /// Send an email
    ///
    /// # Errors
    /// When email sending is failed
    pub async fn send_notification_to_user(
        ctx: &AppContext,
        appointment: &appointments::Model,
    ) -> Result<()> {
        let user = Users::find_by_id(&ctx.db, appointment.user_id).await?;
        let user_timezone = Tz::from_str(&user.timezone).map_err(Error::wrap)?;
        let start_time = appointment.start_time.with_timezone(&user_timezone);

        Self::mail_template(
            ctx,
            &notify_user,
            mailer::Args {
                to: user.email,
                locals: json!({
                    "user_name": user.name,
                    "booker_name": appointment.booker_name,
                    "start_time": start_time,
                    "domain": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Send an email
    ///
    /// # Errors
    /// When email sending is failed
    pub async fn send_notification_to_booker(
        ctx: &AppContext,
        appointment: &appointments::Model,
    ) -> Result<()> {
        let user = Users::find_by_id(&ctx.db, appointment.user_id).await?;
        let booker_timezone = Tz::from_str(&appointment.booker_timezone).map_err(Error::wrap)?;
        let start_time = appointment.start_time.with_timezone(&booker_timezone);

        Self::mail_template(
            ctx,
            &notify_client,
            mailer::Args {
                to: appointment.booker_email.clone(),
                locals: json!({
                    "user_name": user.name,
                    "booker_name": appointment.booker_name,
                    "start_time": start_time,
                    "domain": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}
