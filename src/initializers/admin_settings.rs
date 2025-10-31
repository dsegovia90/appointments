use async_trait::async_trait;
use loco_rs::prelude::*;

use crate::models::admin_settings;

pub struct AdminSettingsInitializer;

#[async_trait]
impl Initializer for AdminSettingsInitializer {
    fn name(&self) -> String {
        "admin-settings".to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> Result<()> {
        let creation_result = admin_settings::ActiveModel::create(&ctx.db).await;

        match creation_result {
            Ok(_) => {
                tracing::info!("Admin settings created successfully.");
                Ok(())
            }
            Err(err) => {
                if let DbErr::Custom(ref msg) = err {
                    let msg = msg.to_string();
                    if msg == "Only one admin setting allowed" {
                        tracing::info!("Admin settings already exist");
                        return Ok(());
                    }
                }
                Err(err.into())
            }
        }
    }
}
