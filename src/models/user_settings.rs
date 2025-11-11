pub use super::_entities::user_settings::{ActiveModel, Entity, Model};
use crate::{
    models::{_entities::user_settings::Column, users::users},
    views::user_settings::UserSettingsProps,
};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use serde::Deserialize;
use validator::ValidationError;
pub type UserSettings = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        Ok(self)
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn get_or_create<C: ConnectionTrait>(db: &C, user: &users::Model) -> Result<Self> {
        if let Some(settings) = UserSettings::find_by_user(db, user).await? {
            Ok(settings)
        } else {
            Ok(ActiveModel::create(db, user).await?)
        }
    }
}

fn validate_user_settings(settings: &Validator) -> Result<(), ValidationError> {
    if settings.start_how_far_from_now_in_minutes > settings.end_how_far_from_now_in_minutes {
        Err(ValidationError::new("start_less_than_end")
            .with_message("Can't be greater than end_how_far_from_now_in_minutes".into()))
    } else {
        Ok(())
    }
}

#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_user_settings"))]
pub struct Validator {
    #[validate(range(min = 0, message = "Can't be smaller than 0."))]
    pub start_how_far_from_now_in_minutes: i32,
    #[validate(range(min = 1, message = "Can't be smaller than 1."))]
    pub end_how_far_from_now_in_minutes: i32,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            start_how_far_from_now_in_minutes: self
                .start_how_far_from_now_in_minutes
                .as_ref()
                .to_owned(),
            end_how_far_from_now_in_minutes: self
                .end_how_far_from_now_in_minutes
                .as_ref()
                .to_owned(),
        })
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create<C: ConnectionTrait>(db: &C, user: &users::Model) -> Result<Model, DbErr> {
        let model = Self {
            user_id: Set(user.id),
            start_how_far_from_now_in_minutes: Set(60),
            end_how_far_from_now_in_minutes: Set(60 * 24 * 14),
            ..Default::default()
        };
        model.insert(db).await
    }

    pub async fn put<C: ConnectionTrait>(
        mut self,
        db: &C,
        props: &UserSettingsProps,
    ) -> Result<Model, DbErr> {
        self.start_how_far_from_now_in_minutes = Set(props.start_how_far_from_now_in_minutes);
        self.end_how_far_from_now_in_minutes = Set(props.end_how_far_from_now_in_minutes);
        self.update(db).await
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_user<C: ConnectionTrait>(
        db: &C,
        user: &users::Model,
    ) -> Result<Option<Model>, DbErr> {
        Self::find()
            .filter(Column::UserId.eq(user.id))
            .one(db)
            .await
    }
}
