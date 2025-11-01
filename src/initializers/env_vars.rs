use async_trait::async_trait;
use loco_rs::{environment::Environment, prelude::*};

pub struct EnvVarsInitializer;

#[async_trait]
impl Initializer for EnvVarsInitializer {
    fn name(&self) -> String {
        "env-vars".to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> Result<()> {
        let filename = match ctx.environment {
            Environment::Production => ".env",
            _ => ".env.development",
        };

        if dotenvy::from_filename(filename).is_ok() {
            tracing::info!("{filename} loaded!");
        } else {
            tracing::warn!("{filename} file not found!");
        }

        Ok(())
    }
}
