use iced::widget::image::Handle;
use iced::{widget, Element, Theme,};
use iced::widget::{button, column, image, row, scrollable, space, text};
use iced::{Application, Command, Settings, executor, window};
// use std::fs::{self, read, read_dir, File, OpenOptions};
use std::{fs, vec};
use std::cmp;
use std::path::{Path, PathBuf};

pub fn start_up() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
        size: iced::Size { width: 800.0f32, height: 480.0f32 },
        resizable: true,
        decorations: true,
        ..Default::default()
        },
    ..Default::default()
    };
    Explore::run(settings)
} 

fn _print_paths(userdir: &str) -> () {
    match fs::read_dir(&userdir) {
        Ok(paths) => {
            for path in paths {
                if let Ok(entry) = path {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        
                        println!("|{}|", file_name);
                    } else {
                        println!("Error: Unable to convert file name to string");
                    }
                } else {
                    println!("Error: Unable to read directory entry");
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };
}

fn _dir_to_vec(user_dir: &Path) -> Vec<String> {
    let mut ret = Vec::<String>::new();

    match fs::read_dir(&user_dir) {
        Ok(paths) => {
            for path in paths {
                if let Ok(entry) = path {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        ret.push(file_name);
                    } else {
                        println!("Error: Unable to convert file name to string");
                    }
                } else {
                    println!("Error: Unable to read directory entry");
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };
    ret
}

fn dir_to_paths(user_dir: &Path) -> Vec<PathBuf> {
    let mut ret = Vec::<PathBuf>::new();

    match fs::read_dir(&user_dir) {
        Ok(paths) => {
            for path in paths {
                if let Ok(entry) = path {
                    ret.push(entry.path());
                    
                } else {
                    println!("Error: Unable to read directory entry");
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };
    ret

}


#[derive(Debug)]
struct Explore {
    path: PathBuf,
    dir: Vec<PathBuf>,
    _error: String,
    file: image::Handle,
    folder: image::Handle,
    format: bool,
}

#[derive(Debug,Clone)]
enum Message {
    BACK,
    Forward(String),
    ICONS,
    LISTS,
}


impl Application for Explore {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;


    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Explore{
            path: fs::canonicalize(PathBuf::from("./")).unwrap(), //gets absolute path
            dir: dir_to_paths(fs::canonicalize(PathBuf::from("./")).unwrap().as_path()),
            _error: String::new(),
            file: Handle::from_path("img/file.png"),
            folder: Handle::from_path("img/Folder.png"),
            format: false,

        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("File Selector")
    }

    fn theme(&self) -> Theme {
        widget::theme::Theme::Dracula
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::BACK => {
                match self.path.pop() {
                    false => todo!("maybe a lil screen shake or popup?"),
                    true => self.dir = dir_to_paths(&self.path),
                }
            },
            Message::Forward(s) => {
                self.path.push(s);
                self.dir = dir_to_paths(&self.path);
            },
            Message::ICONS => self.format = false,
            Message::LISTS => self.format = true,
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {

        let mut c: Vec<Element<'_, Self::Message>> = vec![];
        let mut vec_of_data: Vec<Vec<Element<'_, Self::Message>>> = vec![];


        c.push(space::Space::with_width(10).into());

        let a = text(self.path.to_str().unwrap()).size(18);
        
        match self.format {
            true => {
                self.dir.iter().for_each(
                    |s| 
                    match s.is_file() {
                        true => {
                            c.push(
                            row![
                                text(s.file_name().unwrap().to_str().unwrap()).size(18),
                                image(self.file.clone()).width(20).height(20),
                            ].into()
                            )
                        },
                        false => {
                            c.push(
                            row![
                                text(s.file_name().unwrap().to_str().unwrap()).size(18),
                                image(self.folder.clone()).width(20).height(20),
                            ].into()
                            )
                        },
                    }
                );
            },
            false => {
                self.dir.iter().for_each(
                    |s| 
                    match s.is_file() {
                        true => {
                            c.push(
                            column![
                                image(self.file.clone()).width(70).height(70),
                                text(s.file_name().unwrap().to_str().unwrap().get(..cmp::min(8, s.to_str().unwrap().len())).unwrap()).size(18),
                            ]
                            .align_items(iced::Alignment::Center)
                            .into()
                            )
                        },
                        false => {
                            c.push(
                            column![
                                image(self.folder.clone()).width(70).height(70),
                                text(s.file_name().unwrap().to_str().unwrap().get(..cmp::min(8,s.file_name().unwrap().to_str().unwrap().len())).unwrap()).size(18),
                            ]
                            .align_items(iced::Alignment::Center)
                            .into()
                            )
                        },
                    }
                );
            }
        }

        let b = row![
            button("Back")
            .on_press(Message::BACK),
            button("View").on_press(Message::ICONS),
            button("List").on_press(Message::LISTS),
        ];
       
        // vec_of_data.iter()
        // .for_each(
        //     |&v|
        //     x = x.push(
        //         row(*v).into()
        //     )
        // );
        
        match self.format {
            true => scrollable(column![a,b,column(c).spacing(10)]).into(),
            false => scrollable(column![a,b,row(c).spacing(10)].spacing(10)).into(),
        }   
    }
}