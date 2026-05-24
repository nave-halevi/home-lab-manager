use jsonwebtoken::{EncodingKey, Header, encode, decode, Validation, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive (Serialize, Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
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

pub fn verify_token(token: &str) -> Result<Claims, String> {
    
    let token_secrat = "my_super_secret_key_123" ;
    let token_data= decode::<Claims>(
        token,
        &DecodingKey::from_secret(token_secrat.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(token_data.claims)
}