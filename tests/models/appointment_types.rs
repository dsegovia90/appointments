use appointments::{
    app::App,
    models::{
        appointment_types::{self, CreateOrUpdateAppointmentType},
        users::users,
    },
};
use loco_rs::testing::prelude::*;
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
async fn test_model() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();
    let db = &boot.app_context.db;

    let user = users::Entity::find_by_id(db, 2).await.unwrap();

    let appointment_type = appointment_types::ActiveModel::create(
        db,
        CreateOrUpdateAppointmentType {
            duration_in_minutes: 60,
            display_name: "Test Appointment Type".to_string(),
            user: &user,
        },
    )
    .await
    .unwrap();

    assert_eq!(appointment_type.duration_in_minutes, 60);
}

#[tokio::test]
#[serial]
async fn negative_duration_error() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();
    let db = &boot.app_context.db;

    let user = users::Entity::find_by_id(db, 2).await.unwrap();

    let appointment_type = appointment_types::ActiveModel::create(
        db,
        CreateOrUpdateAppointmentType {
            duration_in_minutes: -60,
            display_name: "Test Appointment Type".to_string(),
            user: &user,
        },
    )
    .await;

    assert!(appointment_type.is_err());
}

#[tokio::test]
#[serial]
async fn duplicate_duration_error() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();
    let db = &boot.app_context.db;

    let user = users::Entity::find_by_id(db, 2).await.unwrap();

    let _appointment_type_1 = appointment_types::ActiveModel::create(
        db,
        CreateOrUpdateAppointmentType {
            duration_in_minutes: 60,
            display_name: "Test Appointment Type".to_string(),
            user: &user,
        },
    )
    .await
    .unwrap();
    let appointment_type_2 = appointment_types::ActiveModel::create(
        db,
        CreateOrUpdateAppointmentType {
            duration_in_minutes: 60,
            display_name: "Test Appointment Type".to_string(),
            user: &user,
        },
    )
    .await;

    assert!(appointment_type_2.is_err());
}
