#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::{
    appointments::{self, Appointments},
    users::users,
};
use loco_rs::prelude::*;

#[debug_handler]
pub async fn read(
    State(ctx): State<AppContext>,
    user: users::Model,
) -> Result<Json<Vec<appointments::Model>>> {
    Ok(Json(Appointments::find_upcoming(&ctx.db, &user).await?))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/appointments/")
        .add("/", get(read))
}
