
//! This example showcases an interactive `Canvas` for drawing Bézier curves.
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub mod vec2;
pub use vec2::{Vec2, vec2};

pub mod curve;
pub use curve::Curve;

pub mod spline;
pub use spline::Spline;

pub mod hermite;
pub use hermite::{Hermite, herm};



fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
struct App {
    curves: Vec<Spline<Hermite>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Clear,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Curve Demo")
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Clear => {
                self.curves.clear();
            },
        };
    }

    fn view(&self) -> Element<Message> {
        column! {
            text("Curve editor").width(Length::Shrink).size(50),
        }
        .padding(20)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}
