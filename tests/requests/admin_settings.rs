use appointments::{app::App, models::users::Users};
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_not_get_admin_settings_as_anonymous() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/admin_settings").await;
        assert_eq!(res.status_code(), 401, "Non-user should be unauthorized.");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_not_get_admin_settings_as_non_admin() {
    request::<App, _, _>(|mut request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();

        let user = Users::find_by_id(&ctx.db, 2).await.unwrap();
        let settings = ctx.config.get_jwt_config().unwrap();
        let jwt = user
            .generate_jwt(&settings.secret, settings.expiration)
            .unwrap();
        request.add_header("Authorization", format!("Bearer {jwt}"));
        let res = request.get("/api/admin_settings").await;
        assert_eq!(
            res.status_code(),
            401,
            "Non-admin user should be unauthorized."
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_settings_as_admin() {
    request::<App, _, _>(|mut request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();

        let user = Users::find_by_id(&ctx.db, 1).await.unwrap();
        let settings = ctx.config.get_jwt_config().unwrap();
        let jwt = user
            .generate_jwt(&settings.secret, settings.expiration)
            .unwrap();
        request.add_header("Authorization", format!("Bearer {jwt}"));
        let res = request.get("/api/admin_settings").await;
        assert_eq!(res.status_code(), 200, "Admin user should be authorized.");
    })
    .await;
}
