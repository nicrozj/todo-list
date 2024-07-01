use cercis::prelude::*;

#[component]
pub fn Page<'a>(title: &'a str, head: Element<'a>, children: Element<'a>) -> Element {
    const META_CONTENT: &str = "witdh=device-width, initial-scale=1.0";
    const GOOGLE_ICONS_URL: &str =
        "https://fonts.googleapis.com/css2?family=Bitter:ital,wght@0,700;1,700&display=swap";

    rsx!(
        doctype {}
        html {
            head {
                meta { charset: "UTF-8" }
                meta {
                    name: "viewport",
                    content: "{META_CONTENT}",
                }
                link {
                    rel: "stylesheet",
                    href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
                }
                script { src: "https://unpkg.com/htmx.org@1.9.12" }
                script { src: "https://cdn.tailwindcss.com" }
                link { rel: "stylesheet", href: "{GOOGLE_ICONS_URL}" }
                head
                title { "{title}" }
            }
            body {
                class: "bg-stone-100 text-stone-600",
                children
            }
        }
    )
}

#[component]
pub fn Icon<'a>(class: Option<&'a str>, children: Element<'a>) -> Element {
    let class = class.unwrap_or_default();

    rsx!(span {
        class: "material-symbols-outlined select-none {class}",

        children
    })
}


// #[component]
// pub fn Search() -> Element {

// }
