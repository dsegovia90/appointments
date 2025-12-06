#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{
    models::{
        user_settings::{self},
        users::users,
    },
    views::user_settings::UserSettingsProps,
};

#[debug_handler]
pub async fn read(
    State(ctx): State<AppContext>,
    user: users::Model,
) -> Result<Json<user_settings::Model>> {
    Ok(Json(
        user_settings::Model::get_or_create(&ctx.db, &user).await?,
    ))
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    user: users::Model,
    Json(body): Json<UserSettingsProps>,
) -> Result<Json<user_settings::Model>> {
    Ok(Json(
        user_settings::Model::get_or_create(&ctx.db, &user)
            .await?
            .into_active_model()
            .put(&ctx.db, &body)
            .await?,
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/user_settings/")
        .add("/", get(read))
        .add("/", post(update))
}
