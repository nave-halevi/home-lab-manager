use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_token(user_id: &str) -> Result<String, String> {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as usize;

    let expitation = current_time + (24 * 60 * 60);

    let my_claims = Claims {
        sub: user_id.to_string(),
        exp: expitation,
    };

    let token_secret = "my_super_secret_key_123";

    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(token_secret.as_ref()),
    )
    .map_err(|e| e.to_string())
}
