use hmac::{Hmac, Mac};
use sha1::{Sha1};
use base64::{Engine as _, engine::general_purpose};
use thiserror::Error;
use anyhow::Result;

use crate::settings::{conf};

type HmacSha1 = Hmac<Sha1>;

#[derive(Error, Debug)]
pub enum KeyError {
    #[error("Invalid HMAC key length")]
    InvalidKeyLength(#[from] sha1::digest::InvalidLength),
}

pub fn get_key_by_path(path: String) -> Result<String, KeyError> {
    if conf().secret_key == "" {
        return Ok("unsafe".to_string());
    }

    let mut mac = HmacSha1::new_from_slice(conf().secret_key.as_bytes())?;
    mac.update(path.as_bytes());

    let hmac_base64 = general_purpose::STANDARD.encode(mac.finalize().into_bytes());
    Ok(hmac_base64.replace('+', "-").replace('/', "_"))
}

pub fn is_valid_key(path: String) -> bool {
    let parts: Vec<&str> = path.split('/').collect();

    let key = parts[1];
    let uri = parts[2..].join("/");

    get_key_by_path(uri).unwrap() == key
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.
    use crate::settings::{Settings};

    #[test]
    fn test_get_key_by_path() {
        Settings {
            secret_key: "MY_KEY".to_string(),
            ..Default::default()
        }.make_current();

        let mut path = "50x50/big.jpg".to_string();
        assert_eq!(get_key_by_path(path).unwrap(), "sMxTvxyS2uudMVBgjPv_YfTFe3E=");

        path = "300x200/http://picsum.photo/500/500.jpg".to_string();
        assert_eq!(get_key_by_path(path).unwrap(), "KScb5yXHfcyeQd4evRzy4xiQoaE=");

        path = "300x200/smart/http://picsum.photo/500/500.jpg".to_string();
        assert_eq!(get_key_by_path(path).unwrap(), "FJd9jVRAhh4rucHcwAlqAJyHyd8=");
    }

    #[test]
    pub fn is_valid_key_when_valid() {
        Settings {
            secret_key: "MY_KEY".to_string(),
            ..Default::default()
        }.make_current();

        let path = "/gOUa7YETwP9XVU4yWP_krzT91og=/670x390/big.jpg".to_string();
        assert_eq!(is_valid_key(path), true);
    }

    #[test]
    pub fn is_valid_key_unsafe() {
        Settings {
            secret_key: "".to_string(),
            ..Default::default()
        }.make_current();

        let mut path = "/unsafe/670x390/big.jpg".to_string();
        assert_eq!(is_valid_key(path), true);

        path = "/with-some-key-here-is-invalid/670x390/big.jpg".to_string();
        assert_eq!(is_valid_key(path), false);
    }

    #[test]
    pub fn is_valid_key_when_invalid() {
        Settings {
            secret_key: "ANY_KEY".to_string(),
            ..Default::default()
        }.make_current();

        let path = "my-invalid-key/50x50/big.jpg".to_string();
        assert_eq!(is_valid_key(path), false);
    }
}
