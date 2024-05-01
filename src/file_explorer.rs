use iced::widget::image::Handle;
use iced::{Element, Theme};
use iced::widget::{button, column, image, row, scrollable, space, text, Button};
use iced::{Application, Command, Settings, executor, window};
// use std::fs::{self, read, read_dir, File, OpenOptions};
use std::{fs, vec};
use std::cmp;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

pub fn start_up() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
        size: iced::Size { width: 780.0f32, height: 480.0f32 },
        resizable: false,
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

#[derive(Debug,Clone)]
struct Explore {
    path: PathBuf,
    dir: Vec<PathBuf>,
    error: String,
    file: image::Handle,
    folder: image::Handle,
    selected: HashSet<PathBuf>,
    moves: bool,
    // it: usize,

}

#[derive(Debug,Clone)]
enum Message {
    BACK,
    Selected(PathBuf),
    MOVE,
    SELECT,
    SUBMIT,
    // CHANGE,
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
            error: String::new(),
            file: Handle::from_path("img/file.png"),
            folder: Handle::from_path("img/Folder.png"),
            selected: HashSet::new(),
            moves: false,
            // it: 0,
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("File Selector")
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::BACK => {
                match self.path.pop() {
                    false => todo!("maybe a lil screen shake or popup?"),
                    true => self.dir = dir_to_paths(&self.path),
                }
            },
            Message::MOVE => self.moves = true,
            Message::SELECT => self.moves = false,
            Message::Selected(pb) => {
                match self.moves {
                    true => {
                        match pb.is_dir() {
                            true => {
                                self.path = pb;
                                self.dir = dir_to_paths(&self.path);
                                self.error = String::new();
                            },
                            false => self.error = String::from("Cannot Move to file"),
                        }
                    },
                    false => {
                        match self.selected.contains(&pb) {
                        true => self.selected.remove(&pb),
                        false => self.selected.insert(pb),
                        };
                    },
             }
            },
            Message::SUBMIT => todo!(),
            // Message::CHANGE => self.it += 1,
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let number_of_columns = 10;

        let mut vec_of_data: Vec<Element<'_, Self::Message>> = Vec::with_capacity(number_of_columns);

        let a = text(self.path.to_str().unwrap()).size(18);


        for column_number in 0..cmp::min(number_of_columns - 1, self.dir.len() - 1) { //Runs at MAX  times
            let mut c: Vec<Element<'_, Self::Message>> = vec![];
            
            if column_number == 0 {
                vec_of_data.push(space::Space::with_width(5).into());
            }

            for row_number in 0..(self.dir.len()/(number_of_columns  + column_number)) + 1 {
                
                // println!("{row_number}: | {number_of_columns}: | {column_number}: |{}",(row_number * (number_of_columns - 1)) + column_number)
                // println!("i: {}| j: {}| j*7 +i: {}",column_number,row_number,(row_number * (number_of_columns-1)) + column_number)
                let x = &self.dir[(row_number * (number_of_columns - 1)) + column_number];

                match x.is_file() {
                    true => {
                        // println!("FILE:\t{}",x.file_name().unwrap().to_str().unwrap());                         
                        c.push(
                            column![
                                Button::new(x.file_name().unwrap().to_str().unwrap().get(..cmp::min(8,x.file_name().unwrap().to_str().unwrap().len())).unwrap())
                                .on_press(Message::Selected(x.to_path_buf())),
                                image(self.file.clone()).width(70).height(70),
                            ]
                            .align_items(iced::Alignment::Center)
                            .into()
                        )
                    },
                    false => {
                        // println!("FOLDER\t{}",x.file_name().unwrap().to_str().unwrap());                         
                        c.push(
                        column![
                            Button::new(x.file_name().unwrap().to_str().unwrap().get(..cmp::min(8, x.file_name().unwrap().to_str().unwrap().len())).unwrap())
                            .on_press(Message::Selected(x.to_path_buf())),
                            image(self.folder.clone()).width(70).height(70),
                        ]
                        .align_items(iced::Alignment::Center)
                        .into()
                        )
                    },
                }
            }
            vec_of_data.push(column(c).into());
        }
        

        let b = row![
            space::Space::with_width(20),
            button("Back").on_press(Message::BACK),
            button("Move").on_press(Message::MOVE),
            button("Select").on_press(Message::SELECT),
            button("SUBMIT").on_press(Message::SUBMIT),

            // button("Change Theme").on_press(Message::CHANGE),
            
            
        ];

        let err = text(&self.error).size(18);

        let mut selected_elements: Vec<Element<'_, Self::Message>> = vec![];
        
        let mut selected_strings = vec![];


        self.selected.iter().for_each(|pb| 
            selected_strings.push(pb.file_name().unwrap().to_str().unwrap())
        );

        selected_strings.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        
        selected_strings.iter().for_each(|s| selected_elements.push(text(s).size(18).into()));
        
        scrollable(column![a,
            err,
            b,
            column(selected_elements).spacing(10),
            space::Space::with_height(20),row(vec_of_data).spacing(10)])
            .width(800).into()
        
    }
}
