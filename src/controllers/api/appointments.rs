#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::{
    models::{
        appointments::{self, Appointments},
        users::users,
    },
    views::appointments::{AppointmentsQueryParams, AppointmentsResponse},
};
use loco_rs::prelude::*;

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

#[debug_handler]
pub async fn cancel_appointment(
    Path(id): Path<i32>,
    user: users::Model,
    State(ctx): State<AppContext>,
) -> Result<Json<appointments::Model>> {
    let appointment = Appointments::find_by_id_and_user(&ctx.db, id, &user).await?;

    Ok(Json(appointment.cancel_appointment(&ctx, &user).await?))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/appointments/")
        .add("/", get(read))
        .add("/cancel/{id}", patch(cancel_appointment))
}
