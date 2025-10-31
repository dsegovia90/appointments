use crate::models::admin_settings::{ActiveModel, Model};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct AdminSettingsParams {
    pub allow_new_registrations: Option<bool>,
    pub google_cloud_api_key: Option<String>,
}

impl AdminSettingsParams {
    pub fn update(&self, item: &mut ActiveModel) {
        if let Some(allow_new_registrations) = self.allow_new_registrations {
            item.allow_new_registrations = Set(allow_new_registrations);
        }

        if self.google_cloud_api_key.is_some() {
            item.google_cloud_api_key = Set(self.google_cloud_api_key.clone());
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
    pub google_cloud_api_key: Option<String>,
}

impl From<Model> for AdminSettingsClientFacing {
    fn from(item: Model) -> Self {
        Self {
            created_at: item.created_at,
            updated_at: item.updated_at,
            allow_new_registrations: item.allow_new_registrations,
            id: item.id,
            google_cloud_api_key: item
                .google_cloud_api_key
                .map(|secret| "*".repeat(secret.len())),
        }
    }
}
