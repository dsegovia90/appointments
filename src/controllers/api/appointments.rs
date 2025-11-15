#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::{
    _entities::appointments::Status,
    appointments::{self, Appointments},
    users::users,
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

const fn default_limit() -> u64 {
    10
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct AppointmentsQueryParams {
    #[serde(default)]
    pub page: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
    pub from_date: Option<Date>,
    pub to_date: Option<Date>,
    pub status: Option<Status>,
    pub appointment_type: Option<i32>,
}

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export)]
pub struct AppointmentsResponse {
    pub appointments: Vec<appointments::Model>,
    pub count: u64,
}

#[debug_handler]
pub async fn read(
    State(ctx): State<AppContext>,
    user: users::Model,
    Query(query): Query<AppointmentsQueryParams>,
) -> Result<Json<AppointmentsResponse>> {
    let (appointments, count) =
        Appointments::find_by_user_with_filters(&ctx.db, &user, query).await?;
    Ok(Json(AppointmentsResponse {
        appointments,
        count,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/appointments/")
        .add("/", get(read))
}
