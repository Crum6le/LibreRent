use dioxus::prelude::*;

mod database;
mod init;
mod inventory_element;
mod login;
mod user;

use crate::inventory_element::Inventory;
use crate::login::logout;
use crate::login::LoginPage;
use crate::user::get_user;

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    LoginPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};

        Ok(dioxus::server::router(App).layer(SessionLayer::new(
            SessionStore::<SessionNullPool>::new(
                None,
                SessionConfig::default().with_table_name("session_table"),
            )
            .await?,
        )))
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS}
        Router::<Route> {}
    }
}
#[component]
fn Home() -> Element {
    let nav = navigator();
    let mut user: Signal<Option<i32>> = use_signal(|| None);
    let mut sidebar = use_signal(|| String::new());
    let mut sidebar_bg = use_signal(|| true);

    let version = env!("CARGO_PKG_VERSION");


    let mut site = use_signal(|| 0);

    const LOGO: Asset = asset!("/assets/favicon.png");

    spawn(async move {
        user.set(get_user().await.unwrap());
        if user().unwrap_or(0) == 0 {
            //nav.push(Route::LoginPage {}); // XXX Hier Kommentieren für DEV mode
        }
    });

    let x = match *site.read() {
        0 => rsx! {},
        1 => Inventory(),
        _ => rsx! {},
    };

    rsx! {
        main {
            class: "h-screen bg-no-repeat bg-cover",
            button {
                type: "button",
                class: "hover:text-orange-500 dark:hover:text-purple-600 p-2 absolute sm:hidden drawer-button dark:text-gray-500",

                onclick: move |_| async move{
                    sidebar.set("translate-x-0".to_string());
                    sidebar_bg.set(false);
                },

                svg {
                    class: "w-8 h-8",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: 24,
                    height: 24,
                    fill: "none",
                    view_box: "0 0 24 24",
                    path {
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_width: 2,
                        d: "M5 7h14M5 12h14M5 17h12M5"
                    }
                    }
            }
            //Navigation Sidebar
            aside {
                class: "fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0 {sidebar} bg-gray-300 dark:bg-slate-800",
                div {
                    class: "h-full px-3 py-4 overflow-y-auto",
                    a {
                        class:"flex times-center ps-2.5 mb-5",
                        img {
                            src: "{LOGO}",
                            class: "h-6 me-3",
                            alt: "logo"
                        },
                        span {
                            class: "self-center text-lg text-heading font-semibold whitespace-nowrap dark:text-purple-600 text-orange-500",
                            "LibreRent"
                        }

                    },
                    ul{
                        class: "space-y-2 font-medium",
                        li {
                            a {
                                class: "flex items-center px-2 py-1.5 rounded-lg text-body hover:text-orange-500 dark:hover:text-purple-600 hover:bg-gray-400/50 dark:hover:bg-slate-900/50 text-gray-900 dark:text-gray-500 group",
                                onclick: move |_| async move {
                                    site.set(0);
                                    sidebar.set("".to_string());
                                    sidebar_bg.set(true);
                                    println!("Navigation to 0");
                                },
                                svg {
                                    class: "w-5 h-5 transition duration-75 group-hover:text-orange-500 dark:group-hover:text-purple-500",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: 24,
                                    height: 24,
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: 2,
                                        d: "M10 6.025A7.5 7.5 0 1 0 17.975 14H10V6.025Z"
                                    }
                                    path {
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: 2,
                                        d: "M13.5 3c-.169 0-.334.014-.5.025V11h7.975c.011-.166.025-.331.025-.5A7.5 7.5 0 0 0 13.5 3Z"
                                    }
                                },
                                span{
                                    class: "ms-3",
                                    "Dashboard"
                                }
                            }
                        },
                        li {
                            a {
                                class: "flex items-center px-2 py-1.5 rounded-lg text-body hover:text-orange-500 dark:hover:text-purple-600 hover:bg-gray-400/50 dark:hover:bg-slate-900/50 text-gray-900 dark:text-gray-500 group",
                                onclick: move |_| async move {
                                    site.set(1);
                                    sidebar.set("".to_string());
                                    sidebar_bg.set(true);
                                    println!("Navigation to 1");
                                },
                                svg {
                                    class: "w-5 h-5 transition duration-75 group-hover:text-orange-500 dark:group-hover:text-purple-500",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: 24,
                                    height: 24,
                                    fill: "none",
                                    view_box: "0 -5 24 24",
                                    path {
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: 2,
                                        d: "M8 8v1h4V8m4 7H4a1 1 0 0 1-1-1V5h14v9a1 1 0 0 1-1 1ZM2 1h16a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1Z"
                                    }
                                },
                                span{
                                    class: "ms-3",
                                    "Inventory"
                                }
                            }
                        }

                    },
                    ul {
                        class: "space-y-2 font-medium border-t border-default pt-4 mt-4 border-gray-900 dark:border-slate-950",
                        li {
                            a {
                                class: "flex items-center px-2 py-1.5 rounded-lg text-body hover:text-orange-500 dark:hover:text-purple-600 hover:bg-gray-400/50 dark:hover:bg-slate-900/50 text-gray-900 dark:text-gray-500 group",
                                onclick: move |_| async move{
                                    site.set(3);
                                    sidebar.set("".to_string());
                                    sidebar_bg.set(true);
                                },
                                svg {
                                    class: "w-5 h-5 transition duration-75 group-hover:text-orange-500 dark:group-hover:text-purple-500",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: 24,
                                    height: 24,
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: 2,
                                        d: "M21 13v-2a1 1 0 0 0-1-1h-.757l-.707-1.707.535-.536a1 1 0 0 0 0-1.414l-1.414-1.414a1 1 0 0 0-1.414 0l-.536.535L14 4.757V4a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v.757l-1.707.707-.536-.535a1 1 0 0 0-1.414 0L4.929 6.343a1 1 0 0 0 0 1.414l.536.536L4.757 10H4a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h.757l.707 1.707-.535.536a1 1 0 0 0 0 1.414l1.414 1.414a1 1 0 0 0 1.414 0l.536-.535 1.707.707V20a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-.757l1.707-.708.536.536a1 1 0 0 0 1.414 0l1.414-1.414a1 1 0 0 0 0-1.414l-.535-.536.707-1.707H20a1 1 0 0 0 1-1Z"
                                    }
                                    path {
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: 2,
                                        d: "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"
                                    }
                                },
                                span{
                                    class: "ms-3",
                                    "Settings"
                                }
                            }
                        }
                        li {
                            a {
                                class: "flex items-center px-2 py-1.5 rounded-lg text-body hover:text-orange-500 dark:hover:text-purple-600 hover:bg-gray-400/50 dark:hover:bg-slate-900/50 text-gray-900 dark:text-gray-500 group",
                                onclick: move |_| async move{
                                    _ = logout().await;
                                    _ = nav.replace(Route::LoginPage {});
                                },
                                svg {
                                    class: "w-5 h-5 transition duration-75 group-hover:text-orange-500 dark:group-hover:text-purple-500",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: 24,
                                    height: 24,
                                    fill: "none",
                                    view_box: "0 -5 24 24",
                                    path {
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: 2,
                                        d: "M4 8h11m0 0-4-4m4 4-4 4m-5 3H3a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h3"
                                    }
                                },
                                span{
                                    class: "ms-3",
                                    "Log Out"
                                }
                            }
                        }
                    }
                    p {
                        class: "dark:text-gray-500 text-gray-900 bottom-0 fixed right-0 p-2 text-xs",
                        "Beta V{version}"
                    }
                }


            },

            div {
                class: "fixed top-0 left-0 z-39 h-screen w-screen bg-slate-950/60 sm:hidden",
                hidden: sidebar_bg,
                onclick: move |_| async move {
                    sidebar.set("".to_string());
                    sidebar_bg.set(true);
                }
            },

            //TODO Hier Components Seperat Schreiben und mit if einfügen
            div {
                class:"sm:ml-64 dark:bg-slate-950 h-screen",
                {x}
            }
        }
    }
}

#[component]
fn Test() -> Element {
    rsx! {
        p { "test" }
    }
}
