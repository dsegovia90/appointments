#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{
    models::{
        appointment_types::{self, AppointmentTypes, CreateOrUpdateAppointmentType},
        users,
    },
    views::appointment_types::{CreateAppointmentTypeParams, UpdateAppointmentTypeParams},
};

#[debug_handler]
pub async fn create(
    State(ctx): State<AppContext>,
    user: users::Model,
    Json(params): Json<CreateAppointmentTypeParams>,
) -> Result<Json<appointment_types::Model>> {
    let appointment_type = appointment_types::ActiveModel::create(
        &ctx.db,
        CreateOrUpdateAppointmentType {
            duration_in_minutes: params.duration_in_minutes,
            display_name: params.display_name,
            user: &user,
        },
    )
    .await?;
    Ok(Json(appointment_type))
}

#[debug_handler]
pub async fn read_all(
    State(ctx): State<AppContext>,
    user: users::Model,
) -> Result<Json<Vec<appointment_types::Model>>> {
    let appointment_types = AppointmentTypes::find_by_user(&ctx.db, &user).await?;
    Ok(Json(appointment_types))
}

#[debug_handler]
pub async fn read_single(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    user: users::Model,
) -> Result<Json<appointment_types::Model>> {
    let appointment_type = AppointmentTypes::find_by_id(&ctx.db, id).await?;

    if appointment_type.user_id != user.id {
        return Err(Error::Unauthorized("Does not belong to user.".to_string()));
    }

    Ok(Json(appointment_type))
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    user: users::Model,
    Path(id): Path<i32>,
    Json(params): Json<UpdateAppointmentTypeParams>,
) -> Result<Json<appointment_types::Model>> {
    let appointment_type = AppointmentTypes::find_by_id(&ctx.db, id).await?;

    if appointment_type.user_id != user.id {
        return Err(Error::Unauthorized("Does not belong to user.".to_string()));
    }

    let updated_appointment_type = appointment_type
        .into_active_model()
        .update_with_params(
            &ctx.db,
            CreateOrUpdateAppointmentType {
                duration_in_minutes: params.duration_in_minutes,
                display_name: params.display_name,
                user: &user,
            },
        )
        .await?;
    Ok(Json(updated_appointment_type))
}

#[debug_handler]
pub async fn destroy(
    State(ctx): State<AppContext>,
    user: users::Model,
    Path(id): Path<i32>,
) -> Result<Response> {
    let appointment_type = AppointmentTypes::find_by_id(&ctx.db, id).await?;

    if appointment_type.user_id != user.id {
        return Err(Error::Unauthorized("Does not belong to user.".to_string()));
    }

    appointment_type.into_active_model().delete(&ctx.db).await?;
    format::empty_json()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/appointment_types/")
        .add("/", post(create))
        .add("/", get(read_all))
        .add("/{id}", get(read_single))
        .add("/{id}", put(update))
        .add("/{id}", delete(destroy))
}
