use fltk::{
    app::{channel, App, Receiver, Scheme, Sender},
    image,
    prelude::{GroupExt, WidgetExt, WindowExt},
    window::Window,
};
use fltk_theme::{ThemeType, WidgetScheme, WidgetTheme};
use rust_embed::RustEmbed;

use crate::{
    constants::WIDGET_PADDING,
    utils::{draw_ui, message_waiting_loop, MainWindow, Message},
};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub struct TodolistRS {
    pub a: App,
    pub m_window: MainWindow,
    pub r: Receiver<Message>,
    pub s: Sender<Message>,
}

impl TodolistRS {
    pub fn new() -> Self {
        let a = App::default().with_scheme(Scheme::Gtk);

        let widget_theme = WidgetTheme::new(ThemeType::Dark);
        widget_theme.apply();

        let widget_scheme = WidgetScheme::new(fltk_theme::SchemeType::Aqua);
        widget_scheme.apply();

        let mut wind = Window::default().with_label("Todolist RS");

        let binding = Asset::get("ferris-logo.png").unwrap();
        let icon = binding.data.as_ref();
        wind.set_icon(Some(image::PngImage::from_data(icon).unwrap()));

        let (s, r) = channel::<Message>();

        let m_window = draw_ui(s);

        wind.set_size(
            m_window.description_input.x() + m_window.description_input.width() + WIDGET_PADDING,
            m_window.create_button.y() + m_window.create_button.height() + WIDGET_PADDING,
        );

        s.send(Message::Filter);

        wind.end();
        wind.show();

        Self { a, m_window, r, s }
    }

    pub fn run(&mut self) {
        message_waiting_loop(self);
    }
}

/*
https://docs.rs/fltk/latest/src/temp_converter2/temp_converter2.rs.html#84

cargo watch -w src -x 'run --bin crud'
*/
