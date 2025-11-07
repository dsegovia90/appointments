use crate::{
    models::{appointment_types::AppointmentTypes, users::CurrentAvailabilityProps},
    our_chrono,
    traits::GenericWindowComparison,
};

pub use super::_entities::appointments::{ActiveModel, Entity, Model};
use super::{_entities::appointments::Column, appointment_types, users};
use chrono::Utc;
use chrono_tz::Tz;
use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, QueryOrder};

pub type Appointments = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl GenericWindowComparison<chrono::DateTime<Utc>> for Model {
    fn start_time(&self) -> chrono::DateTime<Utc> {
        self.start_time.to_utc()
    }
    fn end_time(&self) -> chrono::DateTime<Utc> {
        self.endtime.to_utc()
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn validate_appointment(
        db: &DatabaseConnection,
        appointment_type: &appointment_types::Model,
        user: &users::Model,
        from: &chrono::DateTime<Utc>,
        to: &chrono::DateTime<Utc>,
    ) -> Result<()> {
        let time_delta = *to - *from;

        // Validate duration is same
        if i64::from(appointment_type.duration_in_minutes) != time_delta.num_minutes() {
            return Err(Error::Unauthorized("Invalid window.".to_string()));
        }

        // Validate weekly_availabilities has availability for this window
        let now = our_chrono::utc_now();
        let window = user
            .get_current_availabilities_by_appointment_type(
                db,
                CurrentAvailabilityProps {
                    appointment_type,
                    start_how_far_from_now: *from - now,
                    end_how_far_from_now: *to - now,
                },
            )
            .await?
            .into_iter()
            .nth(0)
            .ok_or_else(|| Error::string("No open weekly_availability window."))?;
        if window.start != *from || window.end != *to {
            return Err(Error::Unauthorized("Invalid window.".to_string()));
        }

        Ok(())
    }

    pub async fn body<C: ConnectionTrait>(&self, db: &C) -> Result<String> {
        let appointment_type = AppointmentTypes::find_by_id(db, self.appointment_type_id).await?;

        Ok(format!(
            "{} with {}",
            appointment_type.display_name, self.booker_name
        ))
    }
}

pub struct CreateAppointmentProps<'a> {
    pub booker_phone: String,
    pub booker_name: String,
    pub booker_timezone: Tz,
    pub booker_email: String,
    pub start_time: chrono::DateTime<Utc>,
    pub endtime: chrono::DateTime<Utc>,
    pub user: &'a users::Model,
    pub appointment_type: &'a appointment_types::Model,
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create<C>(db: &C, props: CreateAppointmentProps<'_>) -> ModelResult<Model>
    where
        C: ConnectionTrait,
    {
        let active_model = Self {
            booker_name: ActiveValue::set(props.booker_name),
            booker_phone: ActiveValue::set(props.booker_phone),
            booker_timezone: ActiveValue::set(props.booker_timezone.to_string()),
            booker_email: ActiveValue::set(props.booker_email),
            start_time: ActiveValue::set(props.start_time.into()),
            endtime: ActiveValue::set(props.endtime.into()),
            status: ActiveValue::set("Booked".to_string()),
            user_id: ActiveValue::set(props.user.id),
            appointment_type_id: ActiveValue::set(props.appointment_type.id),
            ..Default::default()
        };
        Ok(active_model.insert(db).await?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_upcoming<C>(db: &C, owner: &users::Model) -> ModelResult<Vec<Model>>
    where
        C: ConnectionTrait,
    {
        let booked = Self::find()
            .order_by_desc(Column::StartTime)
            .filter(Column::UserId.eq(owner.id))
            .filter(Column::Status.eq("Booked"))
            .filter(Column::StartTime.gt(our_chrono::utc_now()))
            .all(db)
            .await?;

        Ok(booked)
    }
}
