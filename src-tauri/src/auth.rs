use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::error::{Result, AppError};

const JWT_SECRET: &[u8] = b"gestore-pub-secret-change-in-production";
const JWT_EXPIRY_HOURS: u64 = 24 * 30; // 30 days

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // member_id
    pub role: String,       // "admin" or "user"
    pub username: String,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: u64,
}

pub fn create_token(member_id: &str, role: &str, username: &str) -> Result<AuthToken> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let exp = now + JWT_EXPIRY_HOURS * 3600;

    let claims = Claims {
        sub: member_id.to_string(),
        role: role.to_string(),
        username: username.to_string(),
        exp,
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )?;

    Ok(AuthToken { token, expires_at: exp })
}

pub fn verify_token(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

pub fn extract_token_from_header(auth_header: Option<&str>) -> Result<String> {
    let header = auth_header.ok_or_else(|| AppError::Unauthorized("Missing authorization header".into()))?;
    
    if !header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid authorization header format".into()));
    }

    Ok(header[7..].to_string())
}

// For Tauri commands, we'll pass the token as a parameter
pub fn validate_admin_role(claims: &Claims) -> Result<()> {
    if claims.role != "admin" {
        return Err(AppError::Unauthorized("Admin access required".into()));
    }
    Ok(())
}