use appointments::{app::App, models::admin_settings::AdminSettings};
use loco_rs::testing::prelude::*;
use sea_orm::{EntityTrait, PaginatorTrait};
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn test_model_exists() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();

    let count = AdminSettings::find()
        .count(&boot.app_context.db)
        .await
        .unwrap();

    assert_eq!(count, 1);
}
