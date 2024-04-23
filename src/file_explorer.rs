use iced::{widget, Alignment, Element, Font, Pixels, Theme,};
use iced::widget::{button, column, row, text, text_input, Space};
use iced::{Application, Command, Settings, executor, window};
use std::fs::{read, read_dir, File, OpenOptions};
use std::io::Write;

use std::path::PathBuf;

pub fn start_up() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
        size: iced::Size { width: 600.0f32, height: 260.0f32 },
        resizable: true,
        decorations: true,
        ..Default::default()
        },
    ..Default::default()
    };
    Explore::run(settings)
} 

#[derive(Debug)]
struct Explore {
    path: PathBuf,
    

}
#[derive(Debug,Clone)]
enum Message {
    BACK,
    FORWARD,
}

impl Application for Explore {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;


    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Explore{
            path: PathBuf::from("./"),
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("File Selector")
    }

    fn theme(&self) -> Theme {
        widget::theme::Theme::Dark
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let a = row![];
        // let mut v: = read_dir(&self.path).unwrap().collect();
        // for file in read_dir(&self.path).unwrap() {
        //     v.push(text(file.unwrap().path().display()).size(12));
        // }

        a.into()
    }
}