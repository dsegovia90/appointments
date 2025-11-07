use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

use crate::models::{appointment_types, appointments, weekly_availabilities};
#[allow(unused_imports)]
use crate::{
    controllers, initializers, models::_entities::users, tasks, workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(initializers::env_vars::EnvVarsInitializer),
            Box::new(initializers::admin_settings::AdminSettingsInitializer),
        ])
    }

    fn routes(ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::api::admin_settings::routes(ctx))
            .add_route(controllers::api::appointment_types::routes())
            .add_route(controllers::api::appointments::routes())
            .add_route(controllers::api::auth::routes())
            .add_route(controllers::api::client_facing::routes())
            .add_route(controllers::api::integrations::google_calendar::routes())
            .add_route(controllers::api::weekly_availabilities::routes())
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        truncate_table(&ctx.db, weekly_availabilities::Entity).await?;
        truncate_table(&ctx.db, appointment_types::Entity).await?;
        truncate_table(&ctx.db, appointments::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        db::seed::<weekly_availabilities::ActiveModel>(
            &ctx.db,
            &base
                .join("weekly_availabilities.yaml")
                .display()
                .to_string(),
        )
        .await?;
        db::seed::<appointment_types::ActiveModel>(
            &ctx.db,
            &base.join("appointment_types.yaml").display().to_string(),
        )
        .await?;
        db::seed::<appointments::ActiveModel>(
            &ctx.db,
            &base.join("appointments.yaml").display().to_string(),
        )
        .await?;

        Ok(())
    }
}
