#![windows_subsystem = "windows"]

use iced::font::Family;
use iced::mouse::Cursor;
use iced::theme::{Custom, Palette};
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke, Text};
use iced::widget::{button, column, row, slider, text};
use iced::{
    window, Alignment, Color, Element, Font, Length, Point, Rectangle, Renderer, Size, Task, Theme,
};
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

#[derive(Debug)]
struct Diagram {
    states: [bool; 26],
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
        Message::Tap => {
            state.tap += 1.0;
            state.states[0] = !state.states[0];
            state.states[1] = !state.states[1];
        }
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
        // tap button
        row![button(text("Tap").size(64).align_x(Alignment::Center))
            .on_press(Message::Tap)
            .padding(20)
            .style(|theme, status| {
                let mut style = iced::widget::button::primary(theme, status);
                style.border.radius = 5.0.into();
                style
            })
            .width(Length::Fill),],
        // diagram
        Canvas::new(Diagram {
            states: state.states,
        })
        .width(Length::Fill)
        .height(Length::Fill),
        // threshold slider
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

impl<Message> Program<Message> for Diagram {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let neon = Color::from_rgb8(0, 255, 175);

        let col_w = bounds.width / 8.0;
        let row_h = bounds.height / 8.0;

        let points = [
            Point::new(col_w * 4.0, row_h * 4.0), // a
            Point::new(col_w * 3.0, row_h * 7.0), // b
            Point::new(col_w * 2.0, row_h * 5.0), // c
            Point::new(col_w * 3.0, row_h * 6.0), // d
            Point::new(col_w * 4.0, row_h),       // e
            Point::new(col_w * 5.0, row_h * 3.0), // f
            Point::new(col_w * 2.0, row_h * 2.0), // g
            Point::new(col_w * 7.0, row_h),       // h
            Point::new(col_w * 5.0, row_h),       // i
            Point::new(col_w * 4.0, row_h * 7.0), // j
            Point::new(col_w * 2.0, row_h * 4.0), // k
            Point::new(col_w * 6.0, row_h * 4.0), // l
            Point::new(col_w * 2.0, row_h),       // m
            Point::new(col_w * 3.0, row_h * 4.0), // n
            Point::new(col_w, row_h),             // o
            Point::new(col_w * 5.0, row_h * 6.0), // p
            Point::new(col_w, row_h * 2.0), // q
            Point::new(col_w * 5.0, row_h * 4.0), // r
            Point::new(col_w * 6.0, row_h),       // s
            Point::new(col_w * 3.0, row_h),       // t
            Point::new(col_w * 5.0, row_h * 2.0), // u
            Point::new(col_w * 6.0, row_h * 2.0), // v
            Point::new(col_w * 4.0, row_h * 6.0), // w
            Point::new(col_w * 2.0, row_h * 6.0), // x
            Point::new(col_w, row_h * 4.0), // y
            Point::new(col_w * 2.0, row_h * 3.0), // z
        ];

        let dots = [
            false, // a
            true, // b
            true, // c
            true, // d
            true, // e
            true, // f
            true, // g
            true, // h
            true, // i
            false, // j
            false, // k
            true, // l
            false, // m
            true, // n
            false, // o
            true, // p
            false, // q
            true, // r
            true, // s
            false, // t
            false, // u
            false, // v
            false, // w
            false, // x
            false, // y
            true, // z
        ];
        
        // Lines
        let connects = [
            (14, 7), // o - h
            (24, 13), // y - n
            (0, 11), // a - l
            (16, 6), // q - g
            (23, 3), // x - d
            (22, 15), // w - p
            (12, 25), // m - z
            (19, 1), // t - b
            (4, 9), // e - j
            (10, 2), // k - c
            (8, 5), // i - f
            (18, 21), // s - v
        ];

        for connect in connects {
            let line = Path::line(points[connect.0], points[connect.1]);
            frame.stroke(&line, Stroke::default().with_color(Color::WHITE).with_width(8.0));
        }

        // Dots and dashes
        for i in 0..26 {
            if dots[i] {
                // dots
                let dot = Path::circle(points[i], 40.0);

                if self.states[i] {
                    frame.fill(&dot, neon);
                    frame.stroke(&dot, Stroke::default().with_color(neon).with_width(8.0));
                } else {
                    frame.fill(&dot, Color::BLACK);
                    frame.stroke(&dot, Stroke::default().with_color(neon).with_width(8.0));
                }
            } else {
                // dashes
                let dash = Path::rectangle(
                    Point::new(points[i].x - 40.0, points[i].y - 40.0),
                    Size::new(80.0, 80.0),
                );

                if self.states[i] {
                    frame.fill(&dash, neon);
                    frame.stroke(&dash, Stroke::default().with_color(neon).with_width(8.0));
                } else {
                    frame.fill(&dash, Color::BLACK);
                    frame.stroke(&dash, Stroke::default().with_color(neon).with_width(8.0));
                }
            }

            // Labels
            let letter = (b'A' + i as u8) as char;

            frame.fill_text(Text {
                content: letter.to_string(),
                position: points[i],
                color: if self.states[i] { Color::BLACK } else { neon },
                size: 60.0.into(),
                align_x: text::Alignment::Center,
                align_y: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}
