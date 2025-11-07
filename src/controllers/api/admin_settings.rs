#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    models::{admin_settings::AdminSettings, users::users},
    views::admin_settings::{AdminSettingsClientFacing, AdminSettingsParams},
};
use axum::{
    extract::Request,
    middleware::{self, Next},
};
use loco_rs::prelude::*;

#[debug_handler]
pub async fn read(State(ctx): State<AppContext>) -> Result<Json<AdminSettingsClientFacing>> {
    Ok(Json(AdminSettings::load(&ctx.db).await?.into()))
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<AdminSettingsParams>,
) -> Result<Json<AdminSettingsClientFacing>> {
    let item = AdminSettings::load(&ctx.db).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    Ok(Json(item.into()))
}

async fn must_be_admin(user: users::Model, req: Request, next: Next) -> Result<Response> {
    if !user.is_admin() {
        return Err(Error::Unauthorized("User is not admin.".to_string()));
    }

    Ok(next.run(req).await)
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/admin_settings/")
        .add("/", get(read))
        .add("/", put(update))
        .layer(middleware::from_fn_with_state(ctx.clone(), must_be_admin))
}
