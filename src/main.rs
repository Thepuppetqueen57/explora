use dioxus::prelude::*;
use dioxus_desktop::Config;

fn main() {
    let css = include_str!("style.css");

    let window_title = "Explora Browser";

    let config = Config::new()
        .with_custom_head(format!(
            "<style>{}</style>",
            css
        ))

        .with_window(dioxus_desktop::WindowBuilder::new().with_title(window_title));

    dioxus_desktop::launch_cfg(app, config);
}

fn app(cx: Scope) -> Element {
    let searching = use_state(&cx, || false);
    cx.render(rsx!(
        div {
            class: "search-div",
            form {
                onsubmit: move |event| {
                    println!("Submitted! {:?}", event.values);
                    searching.set(true);
                },
                input { name: "search" }
                input { r#type: "submit", value: "search" }
                {searching.then(|| rsx!(
                    h1 { "Searching.." }
                ))}
            }
        }
    ))
}