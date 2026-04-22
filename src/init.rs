use dioxus::{fullstack::*, prelude::*, CapturedError};
use serde::*;

const BG_IMAGE: Asset = asset!("/assets/login/bg.png");

#[cfg(feature = "server")]
use crate::database::user_db::*;
use crate::Route;

#[cfg(feature = "server")]
use axum_session::{Session, SessionNullPool};

#[get("/init_db")]
async fn init_db() -> Result<()> {
    #[cfg(feature = "server")]
    let _ = init();

    Ok(())
}

/*#[get("/init_user")]
async fn init_user() -> Result<()> {
    #[cfg(feature = "server")]
    let _ = create_user_custom();
    Ok(())
}*/

#[derive(Deserialize, Serialize)]
pub struct InitForm {
    username: String,
    email: String,
    password: String,
    password_rep: String,
}

#[component]
pub fn InitPage() -> Element {
    let nav = navigator();
    let mut response_bool = use_signal(|| true);
    let mut response: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        main { class: "h-screen bg-no-repeat bg-cover",
            background_image: "url({BG_IMAGE})",
            div{
                class: "flex items-center justify-center h-screen bg-linear-0 from-amber-950/0 to-amber-950/50 dark:from-slate-950/25 dark:to-slate-950/75",
                form{
                    onsubmit: move |evt: FormEvent| async move {
                        evt.prevent_default();

                        let values:  InitForm = evt.parsed_values().unwrap();
                        //response.set(login(Form(values)).await.unwrap());

                        //spawn(async move {

                            if values.password != values.password_rep {
                                response.set("Passwords do not match".to_string()); //TODO Add check for specific length and number/letter count
                                response_bool.set(false);
                            } else {
                                if let Err(err) = init_user(values.username, values.email, values.password).await{
                                    response_bool.set(false);
                                    response.set(format!("{err}")); //BUG Error Print is not "beautiful"
                                }
                            }

                            if *response_bool.read() {
                                _ = nav.replace(Route::LoginPage {});
                            }
                        //});


                    },
                    div{

                        class: "bg-white dark:bg-slate-950 w-96 p-6 rounded shadow-sm",
                        p {class: "text-gray-800 dark:text-gray-500 mb-10 mt-2 text-4xl flex justify-center font-bold ", "Init"}
                        p { class: "dark:text-gray-400 bg-red-300/50 dark:bg-red-900/50 shadow-sm rounded p-2 border-3 border-red-500 dark:border-red-800 justify-center mb-1",
                            hidden: response_bool,
                            {response}}
                        p {class: "text-gray-800 dark:text-gray-500 mb-1", "Email" }
                        input { class: "w-full bg-gray-100 dark:bg-slate-800 text-gray-500 px-1 py-2 outline-none mb-6", r#type: "text", name: "email", placeholder: "user@example.com", required: true}
                        p {class: "text-gray-800 dark:text-gray-500 mb-1", "Username" }
                        input { class: "w-full bg-gray-100 dark:bg-slate-800 text-gray-500 px-1 py-2 outline-none mb-6", r#type: "text", name: "username", placeholder: "ex4mple", required: true}
                        p {class: "text-gray-800 dark:text-gray-500 mb-1", "Password" }
                        input { class: "w-full bg-gray-100 dark:bg-slate-800 text-gray-500 px-1 py-2 outline-none mb-8", r#type: "password", name: "password", placeholder: "Your Password", required: true}
                        p {class: "text-gray-800 dark:text-gray-500 mb-1", "Repeat Password" }
                        input { class: "w-full bg-gray-100 dark:bg-slate-800 text-gray-500 px-1 py-2 outline-none mb-8", r#type: "password", name: "password_rep", placeholder: "Your Password", required: true}
                        button { class: "text-gray-100 w-full py-2 rounded bg-linear-70 from-orange-500 to-pink-500 dark:from-purple-500 bg-[size:_150%] bg-[position:_0%_0%] hover:bg-[position:_100%_100%] transition-all delay-150 duration-600 ", r#type: "submit", name:"btn", "Create"}
                    }
                }
            }
        }
    }
}

#[post("/init/user", session: Session<SessionNullPool>)]
async fn init_user(username: String, email: String, password: String) -> Result<()> {
    create_new_user(&username, &email, &password).map_err(CapturedError::from)
}