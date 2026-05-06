#![windows_subsystem = "windows"]

use iced::font::Family;
use iced::mouse::Cursor;
use iced::theme::{Custom, Palette};
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke, Text};
use iced::widget::{button, column, row, slider, text};
use iced::{
    window, Alignment, Color, Element, Font, Length, Point, Rectangle, Renderer, Task, Theme,
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

        let r1_y = bounds.height / 2.0;
        let a_p = Point::new(bounds.width * 0.3, r1_y);
        let b_p = Point::new(bounds.width * 0.7, r1_y);
        let radius = 60.0;

        // 1. Draw Connecting Line
        let line = Path::line(a_p, b_p);
        frame.stroke(&line, Stroke::default().with_color(neon).with_width(4.0));

        // 2. Draw A Circle
        let a_circle = Path::circle(a_p, radius);
        if !self.states[0] {
            frame.fill(&a_circle, neon);
        } else {
            frame.stroke(
                &a_circle,
                Stroke::default().with_color(neon).with_width(4.0),
            );
        }
        
        // 2. Draw B Circle
        let b_circle = Path::circle(b_p, radius);
        if !self.states[0] {
            frame.fill(&b_circle, neon);
        } else {
            frame.stroke(
                &b_circle,
                Stroke::default().with_color(neon).with_width(4.0),
            );
        }

        // 3. Draw Left Text
        frame.fill_text(Text {
            content: 'B'.to_string(),
            position: b_p,
            color: if !self.states[0] { Color::BLACK } else { neon },
            size: 80.0.into(),
            align_x: text::Alignment::Center,
            align_y: iced::alignment::Vertical::Center,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
    
    // fn circle(char, 
}
