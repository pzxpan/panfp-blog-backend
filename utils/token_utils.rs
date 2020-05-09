use crate::models::user_token::{UserToken, KEY};

use jsonwebtoken::{DecodingKey, TokenData, Validation};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(&KEY), &Validation::default())
}
