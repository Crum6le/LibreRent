mod inner {
    #[cfg(feature = "server")]
    pub mod user_db {

        use std::sync::Arc;

        use dioxus::CapturedError;
        use dioxus::Result;
        use rand::Rng;
        use rusqlite::Connection;

        use crate::database::UserAlreadyTakenError;
        thread_local! {

            pub static DB: rusqlite::Connection = {
                Connection::open("database/librerent.db").expect("Failed to open database")
            };
        }

        pub fn init() -> Result<()> {
            match DB.with(|conn| {
                conn.execute(
                    "CREATE TABLE if not exists `user` (
                `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                `username` TEXT NOT NULL,
                `password` BINARY(64) NOT NULL,
                `salt` BINARY(64) NOT NULL,
                `email` TEXT NOT NULL,
                `last_change` DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
                )",
                    (),
                )
            }) {
                Ok(_) => println!("Database created Succesfully"),
                Err(error) => println!("Table Creation Failed with error: {}", error),
            };
            Ok(())
        }

        pub fn create_new_user(username: &str, email: &str, password: &str) -> Result<()> {
            use sha3::{Digest, Sha3_512};
            //use rand::prelude::*;

            let mut rng = rand::rng();
            let mut hasher = Sha3_512::new();

            hasher.update(password);

            let mut salt = [0u8; 64];
            rng.fill(&mut salt[..]);

            hasher.update(salt);
            let password_hash: [u8; 64] = hasher.finalize().into();

            let id = DB.with(|conn| {
                let mut statement = conn
                .prepare("SELECT id FROM user WHERE (username=? OR email=? OR username=? OR email=?)")?;
                let mut rows = statement
                .query((&username, &username, &email, &email))?;

                let mut id: Vec<i32> = Vec::new();

                while let Some(row) = rows.next()? {
                    id.push(row.get(0).unwrap());
                }
                return Ok::<_, rusqlite::Error>(id);
            }).map_err(|err| CapturedError(Arc::new(err.into())))?;

            if id.is_empty() {
                match DB.with(|conn| {
                    conn.execute(
                        "INSERT INTO user (username, password, salt, email) VALUES (?,?,?,?)",
                        (&username, &password_hash, &salt, &email),
                    )
                }) {
                    Ok(_) => {}
                    Err(error) => println!("Insert failed: {}", error),
                };
            } else {
                return Err(CapturedError::new(UserAlreadyTakenError));
            }
            Ok(())
        }
    }
}

pub use inner::*;

#[derive(Debug)]
pub struct UserAlreadyTakenError;
impl core::fmt::Display for UserAlreadyTakenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User already taken.")
    }
}
impl core::error::Error for UserAlreadyTakenError {}
