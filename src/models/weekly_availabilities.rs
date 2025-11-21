use crate::{
    models::users::users, traits::GenericWindowComparison,
    views::admin::WeeklyAvailabilitiesCreateUpdateParams,
};

use super::_entities::weekly_availabilities::Column;
pub use super::_entities::weekly_availabilities::{ActiveModel, Entity, Model};
use chrono::Duration;
use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, QueryOrder, TryIntoModel};
use serde::Deserialize;
use validator::ValidationError;
pub type WeeklyAvailabilities = Entity;

fn validate_range(value: &Validator) -> Result<(), ValidationError> {
    if value.to <= value.from {
        return Err(ValidationError::new("\"from\" must be less than \"to\"."));
    }
    Ok(())
}

#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_range"))]
pub struct Validator {
    #[validate(range(min = 0, max = 10080,))]
    pub from: i32,
    #[validate(range(min = 0, max = 10080,))]
    pub to: i32,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            from: self.from.as_ref().to_owned(),
            to: self.to.as_ref().to_owned(),
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

/// Weekly availability duration in minutes from Monday 00:00
#[derive(Debug)]
pub struct WeeklyAvailabilityDuration {
    pub from: Duration,
    pub to: Duration,
}

impl From<Model> for WeeklyAvailabilityDuration {
    fn from(model: Model) -> Self {
        let from = i64::from(model.from);
        let to = i64::from(model.to);
        Self {
            from: Duration::minutes(from),
            to: Duration::minutes(to),
        }
    }
}

impl GenericWindowComparison<i32> for Model {
    fn start_time(&self) -> i32 {
        self.from
    }
    fn end_time(&self) -> i32 {
        self.to
    }
}

pub struct CreateProps<'a> {
    pub from: i32,
    pub to: i32,
    pub user: &'a users::Model,
}

impl GenericWindowComparison<i32> for CreateProps<'_> {
    fn start_time(&self) -> i32 {
        self.from
    }
    fn end_time(&self) -> i32 {
        self.to
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Clashing periods.")]
    Clash(Model),
    #[error(transparent)]
    ModelError(#[from] ModelError),
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create<C>(db: &C, props: CreateProps<'_>) -> ModelResult<Model>
    where
        C: ConnectionTrait,
    {
        let rest = WeeklyAvailabilities::find_by_user(db, props.user, vec![]).await?;

        if props.clash_check(&rest) {
            return Err(ModelError::Message("Clashing periods.".to_string()));
        }

        let active_model = Self {
            from: sea_orm::Set(props.from),
            to: sea_orm::Set(props.to),
            user_id: sea_orm::Set(props.user.id),
            ..Default::default()
        };
        Ok(active_model.insert(db).await?)
    }

    pub async fn put<C>(
        mut self,
        db: &C,
        props: WeeklyAvailabilitiesCreateUpdateParams,
        user: &users::Model,
    ) -> ModelResult<Model, MyError>
    where
        C: ConnectionTrait,
    {
        let this = self.clone().try_into_model().map_err(ModelError::from)?;
        let rest = WeeklyAvailabilities::find_by_user(db, user, vec![&this]).await?;

        if props.clash_check(&rest) {
            return Err(MyError::Clash(this));
        }

        self.from = sea_orm::Set(props.start_time());
        self.to = sea_orm::Set(props.end_time());

        self.validate()
            .map_err(|e| ModelError::msg(e.to_string().as_str()))?;

        Ok(self.update(db).await.map_err(ModelError::from)?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_user<C>(
        db: &C,
        user: &users::Model,
        exclude: Vec<&Model>,
    ) -> ModelResult<Vec<Model>>
    where
        C: ConnectionTrait,
    {
        let models = Self::find()
            .filter(Column::UserId.eq(user.id))
            .filter(Column::Id.is_not_in(exclude.into_iter().map(|item| item.id)))
            .order_by_asc(Column::From)
            .all(db)
            .await?;
        Ok(models)
    }
}
