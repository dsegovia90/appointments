#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::{google_calendars, users::users};
use axum::response::Redirect;
use loco_rs::prelude::*;
use serde::Deserialize;

#[debug_handler]
pub async fn oauth_url(State(ctx): State<AppContext>, user: users::Model) -> Result<Json<String>> {
    // Return the URL using the struct
    let oauth_url = google_calendars::Model::generate_oauth_url(&ctx, &user).await?;
    Ok(Json(oauth_url.to_string()))
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQueryParams {
    pub code: String,
    pub state: Uuid,
    pub scope: String,
}

#[debug_handler]
pub async fn oauth_callback(
    State(ctx): State<AppContext>,
    Query(params): Query<OAuthCallbackQueryParams>,
) -> Result<Redirect> {
    // Exchange the authorization code for an access token
    let redirect_url = google_calendars::Model::exchange_code_for_token(&ctx, params).await?;

    Ok(Redirect::to(&redirect_url))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/google_calendar/")
        .add("/oauth_url", get(oauth_url))
        .add("/oauth_callback", get(oauth_callback))
}
