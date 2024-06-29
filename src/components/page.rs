use cercis::prelude::*;

#[component]
pub fn Page<'a>(title: &'a str, head: Element, children: Element) -> Element {
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
                script { src: "https://unpkg.com/htmx.org@1.9.12" }
                script { src: "https://cdn.tailwindcss.com" }
                link { rel: "stylesheet", href: "{GOOGLE_ICONS_URL}" }
                head
                title { "{title}" }
            }
            body {
                class: "bg-stone-100",
                children
            }
        }

    )
}

pub fn test() {
    println!("123");
}
