use dioxus::{
    html::{div, header},
    prelude::*,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const HOME_SVG: Asset = asset!("/assets/home.svg");
const SETTINGS_SVG: Asset = asset!("/assets/settings.svg");
const ARROW_SVG: Asset = asset!("/assets/backarrow.svg");
const OPTION_SVG: Asset = asset!("/assets/options.svg");


fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            Router::<Route> {}
        }
    });
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    App {},
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-screen justify-between",
            header {                
                div {
                    class: "border-b flex justify-between h-10",
                    div {
                        class: "w-8 pt-1.5 rounded-full ml-2 hover:bg-gray-200",
                        img { src: ARROW_SVG }
                    },
                    div {
                        // "Title"
                    },
                    div {
                        class: "w-7 pt-1.5 rounded-full hover:bg-gray-200 mr-2",
                        img { src: OPTION_SVG }
                    }
                }
            },
            main {
                class: "flex-grow p-4 overflow-y-auto",
                Card { "Default" }
             },
            MainFooter {}
         }
    }
}


#[component]
fn MainFooter() -> Element {
    rsx! {
        footer {
            div {
                class: "flex justify-around border-t py-2 ",
                div {
                    class: "w-12 border rounded-full p-2 hover:bg-gray-200",
                    img { src: HOME_SVG, class: ""}
                },
                div {
                    class: "w-12 border rounded-full p-2 hover:bg-gray-200",
                    img { src: SETTINGS_SVG, class: ""}
                }
            }
         }
    }
}

#[component]
fn Card(children: Element) -> Element {
    rsx! {
        div {
            class: "flex border rounded-lg px-4 py-2",
            {children}
        }
    }
}