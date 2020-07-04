use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub sub: String,
    pub name: String,
    pub exp: u128,
}

fn get_time() -> u128 {
    let start = SystemTime::now();

    start.duration_since(UNIX_EPOCH).unwrap().as_millis() + 86400 * 3
}

pub fn encode(name: String) -> String {
    let auth_payload = AuthPayload {
        sub: "token".to_owned(),
        name: name,
        exp: get_time(),
    };

    let auth_token = match jwt_encode(
        &Header::default(),
        &auth_payload,
        &EncodingKey::from_secret(&env::var("jwt_secret").unwrap().as_ref()),
    ) {
        Ok(value) => value,
        Err(_) => panic!(),
    };

    return auth_token;
}

pub fn decode(token: String) -> String {
    let validation = Validation {
        sub: Some("token".to_owned()),
        ..Validation::default()
    };

    let auth_token = match jwt_decode::<AuthPayload>(
        &token,
        &DecodingKey::from_secret(&env::var("jwt_secret").unwrap().as_ref()),
        &validation,
    ) {
        Ok(c) => c,
        Err(_) => panic!("Error"),
    };

    return auth_token.claims.name;
}
