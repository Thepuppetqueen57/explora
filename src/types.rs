use raylib::prelude::{ Rectangle, Color };

pub struct Button {
    pub rect: Rectangle,
    pub text: String,
    pub font_size: i32,
    pub hover_scale: f32,
    pub press_offset: f32,
    pub is_pressed: bool,
    pub animation_timer: f32
}

pub struct ButtonColor {
    pub main: Color,
    pub outline: Color,
    pub text: Color
}

pub struct TextBox {
    pub rect: Rectangle,
    pub text: String,
    pub text_size: u8,
    pub max_length: u8,
    pub spaces_allowed: bool,
    pub active: bool
}