use dioxus::prelude::*;

#[component]
pub fn Inventory() -> Element {
    rsx! {
        div {
            class: "p-3 grid grid-rows-5 grid-cols-6 gap-1 h-screen dark:bg-slate-950",

            div {
                class: "bg-gray-300 dark:bg-slate-800 col-start-1 col-span-6 row-span-5",
                input{
                    class: "bg-gray-200 text-gray-900 dark:bg-slate-700 dark:text-gray-500 m-4 mr-0 pl-1 h-8 inline-block",
                    r#type: "text",
                    name: "query",
                    placeholder: "Search"
                },
                button {
                    class: "bg-gray-200 text-gray-900 dark:bg-slate-700 dark:text-gray-500 m-4 pl-2 pr-2 h-8 inline-block hover:bg-orange-500 dark:hover:bg-purple-600 hover:text-gray-300 rounded-lg",
                    "Search"
                },
                div {

                },
                button {

                }


            }
        }
    }
}
