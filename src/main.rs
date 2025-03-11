use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        button {
            onclick: |_| println!("not done yet guys"),
            "explora"
        }
    ))
}