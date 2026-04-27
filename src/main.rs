#![windows_subsystem = "windows"]

use iced::font::{Family};
use iced::theme::{Custom, Palette};
use iced::widget::{button, center, column, text};
use iced::{Alignment, Color, Element, Font, Task, Theme};
use std::sync::Arc;

#[derive(Default)]
struct State {
    count: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement
}

pub fn main() -> iced::Result {
    iced::application(new, update, view)
        .theme(theme)
        .font(include_bytes!("lexend.ttf"))
        .default_font(Font {
            family: Family::Name("Lexend"),
            ..Font::DEFAULT
        })
        .run()
}

fn new() -> State {
    State { count: 0 }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
    }
    Task::none()
}

fn theme(_state: &State) -> Theme {
    Theme::Custom(Arc::new(Custom::new(
        "Dark Mint".to_string(),
        Palette {
            background: Color::from_rgb8(0, 0, 0),
            text: Color::from_rgb8(255, 255, 255),
            primary: Color::from_rgb8(0, 255, 175),
            success: Color::from_rgb8(50, 200, 50),
            danger: Color::from_rgb8(200, 50, 50),
            warning: Color::from_rgb8(255, 180, 50),
        },
    )))
}

fn view(state: &State) -> Element<'_, Message> {
    center(
        column![
            button(text("Increment").size(20))
                .on_press(Message::Increment)
                .padding(20),
            text(format!("{}", state.count)).size(80),
            button(text("Decrement").size(20))
                .on_press(Message::Decrement)
                .padding(20)
        ]
        .spacing(40)
        .align_x(Alignment::Center),
    )
    .into()
}
