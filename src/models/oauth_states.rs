pub use super::_entities::oauth_states::{ActiveModel, Entity, Model};
use crate::models::users;
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type OAuthStates = Entity;

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

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create<C: ConnectionTrait>(db: &C, user: &users::Model) -> Result<Model, DbErr> {
        let active_model = Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            user_id: ActiveValue::Set(user.id),
            ..Default::default()
        };
        active_model.insert(db).await
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    /// Find an `OAuthState` by its UUID and destroy it.
    pub async fn find_by_uuid_and_destroy(
        db: &DatabaseConnection,
        id: &Uuid,
    ) -> Result<Model, DbErr> {
        let db = db.begin().await?;
        let item = Self::find_by_id(*id)
            .one(&db)
            .await?
            .ok_or(DbErr::RecordNotFound("oauth_state not found.".to_string()))?;
        item.clone().into_active_model().delete(&db).await?;
        db.commit().await?;

        Ok(item)
    }
}
