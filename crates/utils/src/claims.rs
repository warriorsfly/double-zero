use crate::constants::BEARER_PREFIX;
use crate::{config::CONFIG, constants::AUTHORIZATION_PREFIX};
use super::Error;
use actix_web::FromRequest;

use chrono::{Duration, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
      /// local_user_id, standard claim by RFC 7519.
  pub sub: i32,
  pub iss: String,
  /// Time when this token was issued as UNIX-timestamp in seconds
  pub iat: i64,
}

impl Claims {
    pub fn new(id: i32) -> Self {
        Self {
            sub:id,
            iss:CONFIG.server.to_string(),
            iat: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        _req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let _auth = _req.headers().get(AUTHORIZATION_PREFIX);

        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split(BEARER_PREFIX).collect();
                let token = _split[1].trim();
                match decode_jwt(token) {
                    Ok(claims) => ok(claims),
                    Err(e) => err(e),
                }
            }
            None => err(Error::Unauthorized(
                "no authorization in headers".to_string(),
            )),
        }
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(claim: Claims) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &claim, &encoding_key)
        .map_err(|e| Error::InternalServerError(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| Error::InternalServerError(e.to_string()))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_creates_a_jwt() {
        let id = 1;
        let private_claim = Claims::new(id);
        let jwt = create_jwt(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let id = 1;
        let private_claim = Claims::new(id);
        let jwt = create_jwt(private_claim.clone()).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
