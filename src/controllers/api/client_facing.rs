#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::{
    extractors::Timezone,
    mailers::appointments::AppointmentsMailer,
    models::{
        appointment_types::{self, AppointmentTypes},
        appointments,
        users::{self, Users},
    },
    views::client_facing::{AvailabilityWindow, BookDay, BookingParams},
};
use axum::debug_handler;
use chrono::{DateTime, TimeDelta};
use chrono_tz::Tz;
use itertools::Itertools;
use loco_rs::prelude::*;
use now::DateTimeNow;
use std::collections::BTreeMap;

#[debug_handler]
async fn appointment_types(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<appointment_types::Model>>> {
    let user = Users::find_by_id(&ctx.db, user_id).await?;
    let appointments = AppointmentTypes::find_by_user(&ctx.db, &user).await?;

    Ok(Json(appointments))
}

#[debug_handler]
async fn availabilities_by_day(
    State(ctx): State<AppContext>,
    Path(appointment_type_id): Path<i32>,
    Timezone(user_timezone): Timezone,
) -> Result<Json<Vec<BookDay>>> {
    let appointment_type = AppointmentTypes::find_by_id(&ctx.db, appointment_type_id).await?;

    let user = Users::find_by_id(&ctx.db, appointment_type.user_id).await?;
    let availabilities = user
        .get_current_availabilities_by_appointment_type(
            &ctx.db,
            users::CurrentAvailabilityProps {
                appointment_type: &appointment_type,
                start_how_far_from_now: TimeDelta::zero(),
                end_how_far_from_now: TimeDelta::days(14),
            },
        )
        .await?;

    let mut days: BTreeMap<DateTime<Tz>, Vec<AvailabilityWindow>> = BTreeMap::new();
    availabilities
        .into_iter()
        .chunk_by(|element| {
            element
                .start
                .with_timezone(&user_timezone)
                .beginning_of_day()
        })
        .into_iter()
        .for_each(|(key, chunk)| {
            let windows: Vec<AvailabilityWindow> = chunk.collect();
            days.entry(key).or_insert(windows);
        });

    let days = days
        .into_iter()
        .map(|(key, chunk)| BookDay {
            day: key.to_utc(),
            availabilities: chunk,
        })
        .collect();

    Ok(Json(days))
}

pub async fn booking(
    State(ctx): State<AppContext>,
    Path(appointment_type_id): Path<i32>,
    Timezone(booker_timezone): Timezone,
    Json(booking): Json<BookingParams>,
) -> Result<Json<()>> {
    let appointment_type = AppointmentTypes::find_by_id(&ctx.db, appointment_type_id).await?;
    let user = users::Entity::find_by_id(&ctx.db, appointment_type.user_id).await?;

    appointments::Model::validate_appointment(
        &ctx.db,
        &appointment_type,
        &user,
        &booking.from,
        &booking.to,
    )
    .await?;

    let appointment = appointments::ActiveModel::create(
        &ctx.db,
        appointments::CreateAppointmentProps {
            booker_phone: booking.booker_phone,
            booker_name: booking.booker_name,
            booker_timezone,
            booker_email: booking.booker_email,
            start_time: booking.from,
            endtime: booking.to,
            user: &user,
            appointment_type: &appointment_type,
        },
    )
    .await?;

    AppointmentsMailer::send_notification_to_booker(&ctx, &appointment).await?;
    AppointmentsMailer::send_notification_to_user(&ctx, &appointment).await?;

    Ok(Json(()))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/client-facing")
        .add("/appointment-types/{user_id}", get(appointment_types))
        .add(
            "/availabilities/{appointment_type_id}",
            get(availabilities_by_day),
        )
        .add("/book/{appointment_type_id}", post(booking))
}
