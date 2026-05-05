#![windows_subsystem = "windows"]

use iced::font::Family;
use iced::theme::{Custom, Palette};
use iced::widget::{button, center, column, container, row, slider, space, text};
use iced::{Alignment, Background, Border, Color, Element, Font, Task, Theme};
use std::sync::Arc;

#[derive(Default)]
struct State {
    tap: f32,
    dot_thresh: f32,
    states: [bool; 26],
}

#[derive(Debug, Clone)]
enum Message {
    Tap,
    DotSlider(f32),
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
    State {
        tap: 0.0,
        dot_thresh: 0.5,
        states: [false; 26],
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Tap => state.tap += 1.0,
        Message::DotSlider(thresh) => state.dot_thresh = thresh,
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
    column![
        // row 1
        row![
            space::horizontal(),
            button(text("Tap").size(20))
                .on_press(Message::Tap)
                .padding(20)
                .style(|theme, status| {
                    let mut style = iced::widget::button::primary(theme, status);
                    style.border.radius = 5.0.into();
                    style
                }),
            space::horizontal(),
        ],
        // row 2
        row![
            space::horizontal(),
            labeled_indicator("A", true, true),
            space::horizontal(),
            labeled_indicator("B", false, true),
            space::horizontal(),
            labeled_indicator("C", true, true),
            space::horizontal(),
        ],
        // row 3
        row![
            space::horizontal(),
            labeled_indicator("D", true, true),
            space::horizontal(),
            labeled_indicator("E", true, true),
            space::horizontal(),
            labeled_indicator("F", true, true),
            space::horizontal(),
        ],
        // row 4
        row![],
        // row 5
        row![],
        // row 6
        row![column![
            text(format!("Dot Threshold: {:.1}", state.dot_thresh)),
            slider(0.0..=1.0, state.dot_thresh, Message::DotSlider).step(0.1)
        ]
        .align_x(Alignment::Center)
        .spacing(20),]
        .padding(20),
    ]
    .into()
}

fn labeled_indicator<'a>(
    label: &'a str,
    is_filled: bool,
    is_circle: bool,
) -> container::Container<'a, Message> {
    let theme_color = Color::from_rgb8(0, 255, 175);

    container(
        text(label)
            .size(80)
            .color(if is_filled { Color::BLACK } else { theme_color }),
    )
    .style(move |_theme| container::Style {
        background: Some(if is_filled {
            Background::Color(theme_color)
        } else {
            Background::Color(Color::WHITE)
        }),
        border: Border {
            radius: if is_circle { 100.0.into() } else { 10.0.into() },
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    })
    .padding(40)
}
