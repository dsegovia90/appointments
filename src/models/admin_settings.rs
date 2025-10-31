pub use super::_entities::admin_settings::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type AdminSettings = Entity;

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
            let count = AdminSettings::find().count(_db).await?;
            if count > 0 {
                return Err(DbErr::Custom("Only one admin setting allowed".to_string()));
            }
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create(db: &impl ConnectionTrait) -> Result<Model, DbErr> {
        let active_model = Self {
            allow_new_registrations: sea_orm::ActiveValue::Set(true),
            ..Default::default()
        };
        active_model.insert(db).await
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
