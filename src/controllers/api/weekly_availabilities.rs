#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{
    models::{
        users,
        weekly_availabilities::{self, CreateProps},
    },
    traits::GenericWindowComparison,
    views::admin::{WeeklyAvailabilitiesCreateUpdateParams, WeeklyAvailabilityByWeekday},
};

#[debug_handler]
pub async fn create(
    State(ctx): State<AppContext>,
    user: users::Model,
    Json(props): Json<WeeklyAvailabilitiesCreateUpdateParams>,
) -> Result<Json<weekly_availabilities::Model>> {
    let model = weekly_availabilities::ActiveModel::create(
        &ctx.db,
        CreateProps {
            from: props.start_time(),
            to: props.end_time(),
            user: &user,
        },
    )
    .await?;

    Ok(Json(model))
}

#[debug_handler]
pub async fn read(
    State(ctx): State<AppContext>,
    user: users::Model,
) -> Result<Json<WeeklyAvailabilityByWeekday>> {
    let weekly_availabilities =
        weekly_availabilities::Entity::find_by_user(&ctx.db, &user, vec![]).await?;

    let normalized_availability_windows: WeeklyAvailabilityByWeekday =
        weekly_availabilities.into_iter().fold(
            WeeklyAvailabilityByWeekday::new(),
            |mut acc, weekly_availability| {
                acc.add_availability_by_day_index(weekly_availability);
                acc
            },
        );

    Ok(Json(normalized_availability_windows))
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    user: users::Model,
    Json(props): Json<WeeklyAvailabilitiesCreateUpdateParams>,
) -> Result<Json<weekly_availabilities::Model>> {
    let model_result = weekly_availabilities::WeeklyAvailabilities::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or(ModelError::EntityNotFound)?
        .into_active_model()
        .put(&ctx.db, props, &user)
        .await;

    match model_result {
        Ok(model) => Ok(Json(model)),
        Err(e) => match e {
            weekly_availabilities::MyError::Clash(model) => Ok(Json(model)),
            weekly_availabilities::MyError::ModelError(err) => Err(err.into()),
        },
    }
}

#[debug_handler]
pub async fn destroy(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    user: users::Model,
) -> Result<Json<WeeklyAvailabilityByWeekday>> {
    let _model = weekly_availabilities::WeeklyAvailabilities::delete_by_id(id)
        .exec(&ctx.db)
        .await?;

    read(State(ctx), user).await
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/weekly_availabilities/")
        .add("/", post(create))
        .add("/", get(read))
        .add("/{id}", put(update))
        .add("/{id}", delete(destroy))
}
