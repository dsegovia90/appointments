use crate::models::{
    _entities::admin_settings::GoogleCalendarSettings,
    admin_settings::{ActiveModel, Model},
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct AdminSettingsParams {
    pub allow_new_registrations: Option<bool>,
    pub google_calendar_settings: Option<GoogleCalendarSettings>,
}

impl AdminSettingsParams {
    pub fn update(&self, item: &mut ActiveModel) {
        if let Some(allow_new_registrations) = self.allow_new_registrations {
            item.allow_new_registrations = Set(allow_new_registrations);
        }

        if self.google_calendar_settings.is_some() {
            item.google_calendar_settings = Set(self.google_calendar_settings.clone());
        }
    }
}

#[derive(Clone, Debug, Serialize, ts_rs::TS)]
#[ts(export)]
pub struct AdminSettingsClientFacing {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub allow_new_registrations: bool,
    pub google_calendar_settings: Option<GoogleCalendarSettings>,
}

impl From<Model> for AdminSettingsClientFacing {
    fn from(item: Model) -> Self {
        Self {
            created_at: item.created_at,
            updated_at: item.updated_at,
            allow_new_registrations: item.allow_new_registrations,
            id: item.id,
            google_calendar_settings: item.google_calendar_settings.map(|settings| {
                GoogleCalendarSettings {
                    google_calendar_api_key: "*".repeat(settings.google_calendar_api_key.len()),
                    google_oauth_client_id: settings.google_oauth_client_id,
                    google_oauth_secret: "*".repeat(settings.google_oauth_secret.len()),
                    google_oauth_redirect_uri_base: settings.google_oauth_redirect_uri_base,
                }
            }),
        }
    }
}
