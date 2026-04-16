use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::database::user_db::*;

#[get("/init_db")]
async fn init_db() -> Result<()> {
    #[cfg(feature = "server")]
    let _ = init();

    Ok(())
}

#[get("/init_user")]
async fn init_user() -> Result<()> {
    #[cfg(feature = "server")]
    let _ = create_user_custom();
    Ok(())
}
