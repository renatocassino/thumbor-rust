use hmac::{Hmac, Mac};
use sha1::{Sha1};
use base64::{Engine as _, engine::general_purpose};
use thiserror::Error;
use anyhow::Result;

type HmacSha1 = Hmac<Sha1>;

#[derive(Error, Debug)]
pub enum KeyError {
    #[error("Invalid HMAC key length")]
    InvalidKeyLength(#[from] sha1::digest::InvalidLength),
}

pub fn get_key_by_path(path: String) -> Result<String, KeyError> {
    let mut mac = HmacSha1::new_from_slice(b"MY_KEY")?;
    mac.update(path.as_bytes());

    let hmac_base64 = general_purpose::STANDARD.encode(mac.finalize().into_bytes());
    Ok(hmac_base64.replace('+', "-").replace('/', "_"))
}

pub fn is_valid_key(key: String, path: String) -> bool {
    get_key_by_path(path).unwrap() == key
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_get_key_by_path() {
        let mut path = "50x50/big.jpg".to_string();
        assert_eq!(get_key_by_path(path).unwrap(), "sMxTvxyS2uudMVBgjPv_YfTFe3E=");

        path = "300x200/http://picsum.photo/500/500.jpg".to_string();
        assert_eq!(get_key_by_path(path).unwrap(), "KScb5yXHfcyeQd4evRzy4xiQoaE=");

        path = "300x200/smart/http://picsum.photo/500/500.jpg".to_string();
        assert_eq!(get_key_by_path(path).unwrap(), "FJd9jVRAhh4rucHcwAlqAJyHyd8=");
    }

    #[test]
    pub fn is_valid_key_when_valid() {
        let key = "sMxTvxyS2uudMVBgjPv_YfTFe3E=".to_string();
        let path = "50x50/big.jpg".to_string();
        assert_eq!(is_valid_key(key, path), true);
    }

    #[test]
    pub fn is_valid_key_when_invalid() {
        let key = "my-invalid-key".to_string();
        let path = "50x50/big.jpg".to_string();
        assert_eq!(is_valid_key(key, path), false);
    }
}