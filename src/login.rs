use dioxus::{fullstack::*, prelude::*};
use serde::*;

#[cfg(feature = "server")]
use axum_session::{Session, SessionNullPool};

#[cfg(feature = "server")]
use crate::user::check_password;
use crate::Route;

const BG_IMAGE: Asset = asset!("/assets/login/bg.png");

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[component]
pub fn LoginPage() -> Element {
    let mut response = use_signal(|| true);
    let nav = navigator();
    rsx! {
        main { class: "h-screen bg-no-repeat bg-cover",
            background_image: "url({BG_IMAGE})",
            div{ class: "flex items-center justify-center h-screen bg-linear-0 from-amber-950/0 to-amber-950/50 dark:from-slate-950/25 dark:to-slate-950/75",
                form {
                    onsubmit: move |evt: FormEvent| async move {
                        evt.prevent_default();

                        let values: LoginForm = evt.parsed_values().unwrap();


                        println!("set to");
                        //response.set(login(Form(values)).await.unwrap());

                        //spawn(async move {
                            let _res = login(Form(values)).await.unwrap();
                            response.set(_res);

                            if *response.read() {
                                _ = nav.replace(Route::Home {});
                            }
                        //});


                    },
                    div{
                        class: "bg-white dark:bg-slate-950 w-96 p-6 rounded shadow-sm",
                        p {class: "text-gray-800 dark:text-gray-500 mb-10 mt-2 text-4xl flex justify-center font-bold ", "Login"}
                        p { class: "dark:text-gray-400 bg-red-300/50 dark:bg-red-900/50 shadow-sm rounded p-2 border-3 border-red-500 dark:border-red-800 justify-center mb-1",
                            hidden: response,
                            "Wrong Password or Username!"}
                        p {class: "text-gray-800 dark:text-gray-500 mb-1", "Username or Email" }
                        input { class: "w-full bg-gray-100 dark:bg-slate-800 text-gray-500 px-1 py-2 outline-none mb-6", r#type: "text", name: "username", placeholder: "user@example.com", required: true}
                        p {class: "text-gray-800 dark:text-gray-500 mb-1", "Password" }
                        input { class: "w-full bg-gray-100 dark:bg-slate-800 text-gray-500 px-1 py-2 outline-none mb-8", r#type: "password", name: "password", placeholder: "Your Password", required: true}
                        button { class: "text-gray-100 w-full py-2 rounded bg-linear-70 from-orange-500 to-pink-500 dark:from-purple-500 bg-[size:_150%] bg-[position:_0%_0%] hover:bg-[position:_100%_100%] transition-all delay-150 duration-600 ", r#type: "submit", name:"btn", "Login"}
                    }
                }
            }
        }
    }
}

#[post("/api/user/login", session: Session<SessionNullPool>)]
async fn login(form: Form<LoginForm>) -> Result<bool> {
    let res = check_password(form.0.username, form.0.password)
        .await
        .expect("Password Checker failed");

    if !res.0 {
        println!("Passwort Falsch");
        return Ok(false);
    } else if res.0 {
        println!("Password richtig id: {}", res.1);
        session.set("user_id", res.1);
        return Ok(true);
    }

    Ok(false)
}

#[post("/api/user/logout", session: Session<SessionNullPool>)]
pub async fn logout() -> Result<()> {
    session.clear();
    session.destroy();
    println!("User Logged out!");
    Ok(())
}
