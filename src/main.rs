use ez_reqwest::get_request;
use raylib::prelude::*;

mod types;
mod funcs;
use types::*;

#[tokio::main]
async fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .resizable()
        .title("Explora Browser")
        .build();

    rl.set_target_fps(60);

    let mut search_button = Button::new(
        0.0,
        20.0,
        100.0,
        50.0,
        "Search",
        20
    );

    let mut url_bar = TextBox {
        rect: Rectangle {
            x: 20.0,
            y: 20.0,
            width: 0.0,
            height: 50.0
        },
        text: "URL".to_string(),
        text_size: 30,
        max_length: 255,
        spaces_allowed: false,
        active: false
    };

    let mut on_new_tab: bool = true;
    let mut url: String = "".to_string();
    let mut website: String = "".to_string();

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        search_button.rect.x = rl.get_screen_width() as f32 - 120.0;
        url_bar.rect.width = rl.get_screen_width() as f32 - 160.0;

        search_button.update(&rl, delta_time);

        if search_button.is_clicked(&rl) && url.len() > 0 {
            on_new_tab = false;

            website = get_request(
                url.clone(),
                None
            ).await;
        }

        if url_bar.is_clicked(&rl) {
            url_bar.active = true
        }

        if url_bar.is_not_clicked(&rl) {
            url_bar.active = false
        }

        url_bar.input(&mut url, &rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color { r:30, g:30, b:30, a:255 });

        search_button.draw(
            false,
            None,
            1.0,
            ButtonColor {
                main: Color::WHITE,
                outline: Color::BLACK,
                text: Color::BLACK
            },
            &mut d
        );

        url_bar.draw(url.clone(), &mut d);

        if !on_new_tab {
            d.draw_text(&website, 20, 100, 30, Color::WHITE);
        }
    }
}