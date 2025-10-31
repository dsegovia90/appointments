use std::sync::LazyLock;

pub use super::_entities::appointment_types::{ActiveModel, Entity, Model};
use crate::models::{_entities::appointment_types::Column, users::users};
use loco_rs::prelude::*;
use regex::Regex;
use sea_orm::entity::prelude::*;
use serde::Deserialize;
use stringcase::kebab_case;
use validator::Validate;
pub type AppointmentTypes = Entity;

static REGEX_KEBAB_CASE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^([a-z0-9]*)(-[a-z0-9]+)*$").unwrap());

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(range(min = 1, max = 1440))]
    pub duration_in_minutes: i32,
    #[validate(length(min = 1, max = 100), regex(path = *REGEX_KEBAB_CASE))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            duration_in_minutes: self.duration_in_minutes.as_ref().to_owned(),
            name: self.name.as_ref().to_owned(),
            display_name: self.display_name.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {}

#[derive(Debug)]
pub struct CreateOrUpdateAppointmentType<'a> {
    pub duration_in_minutes: i32,
    pub display_name: String,
    pub user: &'a users::Model,
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create<C>(db: &C, params: CreateOrUpdateAppointmentType<'_>) -> ModelResult<Model>
    where
        C: ConnectionTrait,
    {
        let active_model = Self {
            duration_in_minutes: sea_orm::ActiveValue::Set(params.duration_in_minutes),
            name: sea_orm::ActiveValue::Set(kebab_case(&params.display_name)),
            display_name: sea_orm::ActiveValue::Set(params.display_name),
            user_id: sea_orm::ActiveValue::Set(params.user.id),
            ..Default::default()
        };
        Ok(active_model.insert(db).await?)
    }

    pub async fn update_with_params<C>(
        mut self,
        db: &C,
        params: CreateOrUpdateAppointmentType<'_>,
    ) -> ModelResult<Model>
    where
        C: ConnectionTrait,
    {
        self.duration_in_minutes = sea_orm::ActiveValue::Set(params.duration_in_minutes);
        self.name = sea_orm::ActiveValue::Set(kebab_case(&params.display_name));
        self.display_name = sea_orm::ActiveValue::Set(params.display_name);

        Ok(self.update(db).await?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_id<C>(db: &C, id: i32) -> ModelResult<Model>
    where
        C: ConnectionTrait,
    {
        Self::find()
            .filter(Column::Id.eq(id))
            .one(db)
            .await?
            .ok_or(ModelError::EntityNotFound)
    }

    pub async fn find_by_duration_in_minutes<C>(
        db: &C,
        duration_in_minutes: i32,
        user: &users::Model,
    ) -> ModelResult<Option<Model>>
    where
        C: ConnectionTrait,
    {
        Ok(Self::find()
            .filter(Column::UserId.eq(user.id))
            .filter(Column::DurationInMinutes.eq(duration_in_minutes))
            .one(db)
            .await?)
    }

    pub async fn find_by_user<C>(db: &C, user: &users::Model) -> ModelResult<Vec<Model>>
    where
        C: ConnectionTrait,
    {
        Ok(Self::find()
            .filter(Column::UserId.eq(user.id))
            .all(db)
            .await?)
    }
}
