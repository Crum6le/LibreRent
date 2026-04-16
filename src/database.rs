
mod inner {
    #[cfg(feature = "server")]
    pub mod user_db {

        use rusqlite::{Connection, Result};
        thread_local! {

            pub static DB: rusqlite::Connection = {
                Connection::open("database/mysoft.db").expect("Failed to open database")
            };
        }

        pub fn init() -> Result<()> {
            match DB.with(|conn| conn.execute(
                "CREATE TABLE if not exists `user` (
                `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                `username` TEXT NOT NULL,
                `password` BINARY(64) NOT NULL,
                `salt` BINARY(64) NOT NULL,
                `email` TEXT NOT NULL,
                `last_change` DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
                )",
                (),
            )) {
                Ok(_) => println!("Database created Succesfully"),
                Err(error) => println!("Table Creation Failed with error: {}", error),
            };
            Ok(())
        }

        pub fn create_user_custom() -> Result<()> {
            use sha3::{Digest, Sha3_512};
            //use rand::prelude::*;

            //let mut rng = rand::rng();
            let mut hasher = Sha3_512::new();

            hasher.update("password");

            let mut salt = [0u8; 64];
            //rng.fill(&mut salt[..]);

            hasher.update(salt);
            let password: [u8; 64] = hasher.finalize().into();

            match DB.with(|conn| conn.execute(
                "INSERT INTO user (username, password, salt, email) VALUES (?,?,?,?)",
                ("username", &password, &salt, "email"),
            )) {
                Ok(_) => {}
                Err(error) => println!("Insert failed: {}", error),
            };
            Ok(())
        }
    }
}

pub use inner::*; 