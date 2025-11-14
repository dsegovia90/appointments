#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::{
    _entities::appointments::Status,
    appointments::{self, Appointments},
    users::users,
};
use loco_rs::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppointmentsQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub from_date: Option<DateTimeUtc>,
    pub to_date: Option<DateTimeUtc>,
    pub status: Option<Status>,
    pub appointment_type: Option<i32>,
}

#[debug_handler]
pub async fn read(
    State(ctx): State<AppContext>,
    user: users::Model,
    Query(query): Query<AppointmentsQueryParams>,
) -> Result<Json<Vec<appointments::Model>>> {
    Ok(Json(
        Appointments::find_by_user_with_filters(&ctx.db, &user, query).await?,
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/appointments/")
        .add("/", get(read))
}
