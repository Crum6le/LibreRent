#[cfg(feature = "server")]
use std::sync::Arc;

use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::CapturedError;

//#[cfg(feature = "server")]
//use rusqlite::{Rows};

#[cfg(feature = "server")]
use axum_session::{Session, SessionNullPool};

#[cfg(feature = "server")]
use crate::database::user_db::DB;

#[cfg(feature = "server")]
pub async fn check_password(username: String, password: String) -> Result<(bool, i32)> {
    //println!("Trying to login {} with passphrase {}", username, password);

    use sha3::{Digest, Sha3_512};

    //let conn = Connection::open("database/librerent.db").expect("Failed to open Database");

    let mut hasher = Sha3_512::new();

    hasher.update(password);

    let (id, salt, password) = DB
        .with(|conn| {
            let mut statement =
                conn.prepare("SELECT id, salt, password FROM user WHERE (username=? OR email=?)")?;
            let mut rows = statement.query((&username, &username))?;

            let mut id: Vec<i32> = Vec::new();
            let mut salt: Vec<[u8; 64]> = Vec::new();
            let mut password: Vec<[u8; 64]> = Vec::new();

            while let Some(row) = rows.next()? {
                id.push(row.get(0).unwrap());
                salt.push(row.get(1).unwrap());
                password.push(row.get(2).unwrap());
            }
            return Ok::<_, rusqlite::Error>((id, salt, password));
        })
        .map_err(|err| CapturedError(Arc::new(err.into())))?;

    if id.is_empty() {
        Ok((false, 0))
    } else {
        hasher.update(salt[0]);
        let hash: [u8; 64] = hasher.finalize().into();

        if hash == password[0] {
            Ok((true, id[0]))
        } else {
            Ok((false, 0))
        }
    }

    //return Ok((false, 0));
}

#[post("/api/get_user", session: Session<SessionNullPool>)]
pub async fn get_user() -> Result<Option<i32>> {
    let user_id: Option<i32> = session.get("user_id");

    Ok(user_id)
}
