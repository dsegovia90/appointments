use appointments::{
    app::App,
    models::{users, weekly_availabilities},
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

    // query your model, e.g.:
    //
    // let item = models::posts::Model::find_by_pid(
    //     &boot.app_context.db,
    //     "11111111-1111-1111-1111-111111111111",
    // )
    // .await;

    // snapshot the result:
    // assert_debug_snapshot!(item);
}

#[tokio::test]
#[serial]
async fn test_out_of_range() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();
    let db = &boot.app_context.db;

    let user = users::Entity::find_by_id(db, 1).await.unwrap();

    let first = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: -10,
            to: 1000,
            user: &user,
        },
    )
    .await;

    dbg!(&first);

    assert!(first.is_err());

    let second = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: 0,
            to: 10081,
            user: &user,
        },
    )
    .await;

    assert!(second.is_err());

    let third = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: 800,
            to: 700,
            user: &user,
        },
    )
    .await;

    assert!(third.is_err());
}

#[tokio::test]
#[serial]
async fn test_clash() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();
    let db = &boot.app_context.db;
    let user = users::Entity::find_by_id(db, 2).await.unwrap();

    let first = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: 800,
            to: 1000,
            user: &user,
        },
    )
    .await;

    assert!(first.is_ok());

    let from_clash = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: 900,
            to: 1200,
            user: &user,
        },
    )
    .await;

    assert!(from_clash.is_err());

    let to_clash = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: 600,
            to: 900,
            user: &user,
        },
    )
    .await;

    assert!(to_clash.is_err());

    let wrap_clash = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: 600,
            to: 1200,
            user: &user,
        },
    )
    .await;

    assert!(wrap_clash.is_err());

    let inner_clash = weekly_availabilities::ActiveModel::create(
        db,
        weekly_availabilities::CreateProps {
            from: 850,
            to: 900,
            user: &user,
        },
    )
    .await;

    assert!(inner_clash.is_err());
}
