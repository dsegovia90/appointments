use axum::{extract::FromRequestParts, http::{HeaderMap, request::Parts}};
use chrono_tz::{ParseError, Tz};

pub struct Timezone(pub Tz);

impl<S> FromRequestParts<S> for Timezone
where
    S: Send + Sync,
{
    type Rejection = loco_rs::errors::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
                   .await
                   .map_err(|err| loco_rs::errors::Error::Message(err.to_string()))?;

        let timezone = headers
            .get("timezone")
            .map(|item| item.to_str())
            .ok_or_else(|| loco_rs::errors::Error::string("Please provide timezone header."))?;

        let timezone: Tz = timezone
            .map_err(|_e| loco_rs::errors::Error::Message("Could not parse timezone.".to_string()))?
            .parse()
            .map_err(|_e: ParseError| loco_rs::errors::Error::Message("Invalid timezone".into()))?;

        Ok(Self(timezone))
    }
}
