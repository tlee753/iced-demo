#![windows_subsystem = "windows"]

use iced::font::Family;
use iced::theme::{Custom, Palette};
use iced::widget::{button, column, container, row, slider, space, text};
use iced::{window, Alignment, Background, Border, Color, Element, Font, Settings, Task, Theme};
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
        .window(window::Settings {
            maximized: true,
            ..Default::default()
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
            button(text("Tap").size(64))
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
            indicator('O', false, false),
            space::horizontal(),
            indicator('M', false, false),
            space::horizontal(),
            indicator('T', false, false),
            space::horizontal(),
            indicator('E', false, false),
            space::horizontal(),
            indicator('I', false, true),
            space::horizontal(),
            indicator('S', false, true),
            space::horizontal(),
            indicator('H', false, true),
            space::horizontal(),
        ],
        // row 3
        row![
            space::horizontal(),
            indicator('Q', false, false),
            space::horizontal(),
            indicator('G', false, true),
            space::horizontal(),
            indicator('U', false, false),
            space::horizontal(),
            indicator('V', false, false),
            space::horizontal(),
        ],
        // row 4
        row![
            space::horizontal(),
            indicator('Z', false, true),
            space::horizontal(),
            indicator('F', false, true),
            space::horizontal(),
        ],
        // row 5
        row![
            space::horizontal(),
            indicator('Y', false, false),
            space::horizontal(),
            indicator('K', false, false),
            space::horizontal(),
            indicator('N', false, true),
            space::horizontal(),
            indicator('A', false, false),
            space::horizontal(),
            indicator('R', false, true),
            space::horizontal(),
            indicator('L', false, true),
            space::horizontal(),
        ],
        // row 6
        row![
            space::horizontal(),
            indicator('C', false, true),
            space::horizontal(),
        ],
        // row 7
        row![
            space::horizontal(),
            indicator('X', false, true),
            space::horizontal(),
            indicator('D', false, true),
            space::horizontal(),
            indicator('W', false, true),
            space::horizontal(),
            indicator('P', false, true),
            space::horizontal(),
        ],
        // row 8
        row![
            space::horizontal(),
            indicator('B', false, true),
            space::horizontal(),
            indicator('J', true, true),
            space::horizontal(),
        ],
        // row 9
        row![column![
            text(format!("Dot Threshold: {:.1}", state.dot_thresh)).size(16),
            slider(0.0..=1.0, state.dot_thresh, Message::DotSlider).step(0.1)
        ]
        .align_x(Alignment::Center)
        .spacing(20),]
        .padding(20),
    ]
    .into()
}

fn indicator(label: char, toggle: bool, circle: bool) -> container::Container<'static, Message> {
    let theme_color = Color::from_rgb8(0, 255, 175);

    container(
        text(label)
            .size(32)
            .color(if toggle { Color::BLACK } else { theme_color }),
    )
    .style(move |_theme| container::Style {
        background: Some(if toggle {
            Background::Color(theme_color)
        } else {
            Background::Color(Color::TRANSPARENT)
        }),
        border: Border {
            radius: if circle { 100.0.into() } else { 10.0.into() },
            width: 10.0,
            color: if toggle { Color::WHITE } else { theme_color },
        },
        ..Default::default()
    })
    .padding(40)
}
