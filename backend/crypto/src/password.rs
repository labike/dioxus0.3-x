use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use password_hash::{Encoding, PasswordHashString};
use tracing::instrument;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("hash error: {0}")]
    HashError(String),

    #[error("password doesn't match")]
    WrongPassword,
}

#[instrument(level = "trace", err, skip_all)]
pub fn hash_password_with_salt<T: AsRef<str>>(
    password: T,
    salt: &SaltString,
) -> Result<PasswordHashString, Error> {
    let password = password.as_ref().as_bytes();

    let argon2 = Argon2::default();

    argon2
        .hash_password(password, salt)
        .map(|hash| hash.serialize())
        .map_err(|err| Error::HashError(err.to_string()))
}

#[tracing::instrument(level = "trace", err, skip_all)]
pub fn hash_password<T: AsRef<str>>(password: T) -> Result<PasswordHashString, Error> {
    let salt = new_salt();
    hash_password_with_salt(password, &salt)
}

#[tracing::instrument(level = "debug", err, skip_all)]
pub fn verify_password<T>(password: T, hash: &PasswordHash) -> Result<(), Error>
where
    T: AsRef<str>,
{
    let password = password.as_ref().as_bytes();

    let argon2 = Argon2::default();

    argon2
        .verify_password(password, hash)
        .map_err(|_| Error::WrongPassword)
}

pub fn new_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

#[tracing::instrument(level = "trace", err, skip_all)]
pub fn deserialize_hash(hash: &str) -> Result<PasswordHash<'_>, Error> {
    PasswordHash::parse(hash, Encoding::B64).map_err(|err| Error::HashError(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_password_with_salt() {
        let password = "password";
        let salt = new_salt();
        let hashed = hash_password_with_salt(password, &salt).expect("failed to hash password");
        assert_ne!(hashed.as_str(), password);
        assert_eq!(hashed.encoding(), Encoding::B64);
    }

    #[test]
    fn matching_passwords_are_properly_verified() {
        let password = "password";
        let hashed = hash_password(password).unwrap();
        let hashed = PasswordHash::parse(hashed.as_str(), Encoding::B64).unwrap();
        verify_password(password, &hashed).expect("failed to verify password");
    }

    #[test]
    fn mismatching_passwords_fail_verification() {
        let password = "password";
        let hashed = hash_password(password).unwrap();
        let hashed = PasswordHash::parse(hashed.as_str(), Encoding::B64).unwrap();
        assert!(verify_password("wrong", &hashed).is_err());
    }

    #[test]
    fn deserializes() {
        let password = "password";
        let hashed = hash_password(password).unwrap();
        assert!(deserialize_hash(hashed.as_str()).is_ok());
    }
}
