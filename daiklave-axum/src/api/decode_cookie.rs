use axum::http::StatusCode;
use axum_extra::extract::SignedCookieJar;
use serenity::all::{UserId};

/// Utility function to try to get a UserId out of a SignedCookieJar. If
/// unsuccessful, returns 401 UNAUTHORIZED to force reauthentication.
pub fn decode_user_id_cookie(jar: SignedCookieJar) -> Result<UserId, StatusCode> {
    // Try to get "daiklaveAuth" cookie from the jar
    let cookie = jar.get("daiklaveAuth").ok_or(StatusCode::UNAUTHORIZED)?;
    // Decode the hex string into byte vector
    let bytes_vec = hex::decode(cookie.value()).or(Err(StatusCode::UNAUTHORIZED))?;
    // Turn the byte vector into an 8-byte array
    let bytes_array = if bytes_vec.len() == 8 {
        bytes_vec.into_iter().enumerate().fold([0; 8], |mut arr, (i, byte)| {
            arr[i] = byte;
            arr
        })
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Turn the big-endian byte array into a u64
    let id_integer = u64::from_be_bytes(bytes_array);
    
    // Turn the u64 into a UserId snowflake and return it
    Ok(UserId::from(id_integer))
}