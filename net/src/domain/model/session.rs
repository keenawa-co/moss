use std::collections::HashSet;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use chrono::Utc;
use http::HeaderValue;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use types::id::NanoId;

use crate::{config::MAGIC_TOKEN_KEY, domain, domain::model::result::Result};

use super::{project::ProjectMeta, result::ResultExtension, OptionExtension};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct SessionInfoEntity {
    pub id: NanoId,
    pub project_meta_id: NanoId,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct SessionEntity {
    pub id: NanoId,
    pub project_meta: Option<ProjectMeta>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SessionTokenClaims {
    #[serde(rename = "sid")]
    pub session_id: NanoId,

    #[serde(rename = "pid")]
    pub project_id: NanoId,

    #[serde(rename = "t")]
    pub timestamp: i64,
}

impl SessionTokenClaims {
    pub fn decode(token_str: &str) -> Result<Self> {
        let magic_key = MAGIC_TOKEN_KEY
            .get()
            .ok_or_config_invalid("Session token MAGIC_KEY was not defined", None)?;

        let token_data: TokenData<SessionTokenClaims> = jsonwebtoken::decode(
            token_str,
            &DecodingKey::from_secret(magic_key.as_ref()),
            &Self::validation(),
        )?;

        Ok(token_data.claims)
    }

    pub fn validation() -> jsonwebtoken::Validation {
        let mut v = Validation::new(Algorithm::HS256);

        v.required_spec_claims =
            HashSet::from([String::from("sid"), String::from("pid"), String::from("t")]);
        v.leeway = 60;
        v.reject_tokens_expiring_in_less_than = 0;
        v.validate_exp = false;
        v.validate_nbf = false;
        v.validate_aud = true;
        v.aud = None;
        v.iss = None;

        return v;
    }
}

impl TryFrom<&HeaderValue> for SessionTokenClaims {
    type Error = domain::model::error::Error;

    fn try_from(value: &HeaderValue) -> std::prelude::v1::Result<Self, Self::Error> {
        let token_str = value
            .to_str()
            .ok_or_resource_invalid("Session token in incorrect format", None)?;

        let claims = SessionTokenClaims::decode(token_str)
            .ok_or_resource_invalid("Failed to decode session token", None)?;

        Ok(claims)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionToken(String);

impl TryFrom<SessionEntity> for SessionToken {
    type Error = domain::model::error::Error;

    fn try_from(value: SessionEntity) -> Result<Self, Self::Error> {
        let project_meta = value
            .project_meta
            .ok_or_resource_invalid("Session project was not found", None)?;

        let claims = SessionTokenClaims {
            session_id: value.id,
            project_id: project_meta.id,
            timestamp: Utc::now().timestamp(),
        };

        let magic_key = MAGIC_TOKEN_KEY
            .get()
            .ok_or_config_invalid("Session token MAGIC_KEY was not defined", None)?;

        Ok(SessionToken(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(magic_key.as_ref()),
        )?))
    }
}

impl std::fmt::Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[Scalar]
impl ScalarType for SessionToken {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = &value {
            Ok(SessionToken(value.to_string()))
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.clone())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct Session {
    pub id: NanoId,
    pub token: SessionToken,
    pub project_meta: Option<ProjectMeta>,
    pub created_at: i64,
}
