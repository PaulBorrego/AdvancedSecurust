use iced::widget::image::Handle;
use iced::{widget, Element, Theme};
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
        size: iced::Size { width: 800.0f32, height: 480.0f32 },
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

#[derive(Debug)]
struct Explore {
    path: PathBuf,
    dir: Vec<PathBuf>,
    _error: String,
    file: image::Handle,
    folder: image::Handle,
    format: bool,
    selected: HashSet<PathBuf>,
    moves: bool,
}

#[derive(Debug,Clone)]
enum Message {
    BACK,
    ICONS,
    LISTS,
    Selected(PathBuf),
    MOVE,
    SELECT,
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
            selected: HashSet::new(),
            moves: false,
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
            Message::ICONS => self.format = false,
            Message::LISTS => self.format = true,
            Message::MOVE => self.moves = true,
            Message::SELECT => self.moves = false,
            Message::Selected(pb) => {
                match self.moves {
                    true => {
                        self.path = pb;
                        self.dir = dir_to_paths(&self.path);
                    },
                    false => {
                        match self.selected.contains(&pb) {
                        true => self.selected.remove(&pb),
                        false => self.selected.insert(pb),
                        };
                    },
             }
            },
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let number_of_columns = 10;

        // let mut c: Vec<Element<'_, Self::Message>> = vec![];
        let mut vec_of_data: Vec<Element<'_, Self::Message>> = Vec::with_capacity(number_of_columns);



        let a = text(self.path.to_str().unwrap()).size(18);


        match self.format {
            true => todo!(), 
            // {
            //     for column_number in 0..cmp::min(number_of_columns - 1, self.dir.len() - 1) { //Runs at MAX 8 times
            //         let mut c: Vec<Element<'_, Self::Message>> = vec![];
                    
            //         if column_number == 0 {
            //             c.push(space::Space::with_width(10).into());
            //         }

            //         for row_number in 0..(self.dir.len()/(number_of_columns + column_number)) + 1 {

            //             // println!("i: {}| j: {}| j*7 +i: {}",column_number,row_number,(row_number * (number_of_columns-1)) + column_number);

            //             let x = &self.dir[(row_number * (number_of_columns - 1)) + column_number];
            //             match x.is_file() {
            //                 true => {
            //                     c.push(
            //                     column![
            //                         image(self.file.clone()).width(70).height(70),
            //                         text(x.file_name().unwrap().to_str().unwrap().get(..cmp::min(8, x.to_str().unwrap().len())).unwrap()).size(18),
            //                     ]
            //                     .align_items(iced::Alignment::Center)
            //                     .into()
            //                     )
            //                 },
            //                 false => {
            //                     c.push(
            //                     column![
            //                         image(self.folder.clone()).width(70).height(70),
            //                         text(x.file_name().unwrap().to_str().unwrap().get(..cmp::min(8,x.file_name().unwrap().to_str().unwrap().len())).unwrap()).size(18),
            //                     ]
            //                     .align_items(iced::Alignment::Center)
            //                     .into()
            //                     )
            //                 },                            
            //             }
                        
            //         }
            //         vec_of_data.push(column(c).into());
            //     }
            // },  
            
            false =>  {
                for column_number in 0..cmp::min(number_of_columns - 1, self.dir.len() - 1) { //Runs at MAX 8 times
                    let mut c: Vec<Element<'_, Self::Message>> = vec![];
                    
                    if column_number == 0 {
                        c.push(space::Space::with_width(10).into());
                    }

                    for row_number in 0..(self.dir.len()/(number_of_columns  - 1 + column_number)) + 1 {

                        // println!("i: {}| j: {}| j*7 +i: {}",column_number,row_number,(row_number * (number_of_columns-1)) + column_number);

                        let x = &self.dir[(row_number * (number_of_columns - 1)) + column_number];
                        match x.is_file() {
                            true => {                            
                                c.push(
                                    column![
                                        Button::new(x.file_name().unwrap().to_str().unwrap().get(..cmp::min(8, x.to_str().unwrap().len())).unwrap())
                                        .on_press(Message::Selected(x.to_path_buf())),
                                        image(self.file.clone()).width(70).height(70),
                                    ]
                                    .align_items(iced::Alignment::Center)
                                    .into()
                                )
                            },
                            false => {
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
            },
        }
        // match self.format {
        //     true => {
        //         self.dir.iter().for_each(
        //             |s| 
        //             match s.is_file() {
        //                 true => {
        //                     c.push(
        //                     row![
        //                         text(s.file_name().unwrap().to_str().unwrap()).size(18),
        //                         image(self.file.clone()).width(20).height(20),
        //                     ].into()
        //                     )
        //                 },
        //                 false => {
        //                     c.push(
        //                     row![
        //                         text(s.file_name().unwrap().to_str().unwrap()).size(18),
        //                         image(self.folder.clone()).width(20).height(20),
        //                     ].into()
        //                     )
        //                 },
        //             }
        //         );
        //     },
        //     false => {
        //         self.dir.iter().for_each(
        //             |s| 
        //             match s.is_file() {
        //                 true => {
        //                     c.push(
        //                     column![
        //                         image(self.file.clone()).width(70).height(70),
        //                         text(s.file_name().unwrap().to_str().unwrap().get(..cmp::min(8, s.to_str().unwrap().len())).unwrap()).size(18),
        //                     ]
        //                     .align_items(iced::Alignment::Center)
        //                     .into()
        //                     )
        //                 },
        //                 false => {
        //                     c.push(
        //                     column![
        //                         image(self.folder.clone()).width(70).height(70),
        //                         text(s.file_name().unwrap().to_str().unwrap().get(..cmp::min(8,s.file_name().unwrap().to_str().unwrap().len())).unwrap()).size(18),
        //                     ]
        //                     .align_items(iced::Alignment::Center)
        //                     .into()
        //                     )
        //                 },
        //             }
        //         );
        //     }
        // }

        let b = row![
            button("Back")
            .on_press(Message::BACK),
            button("View").on_press(Message::ICONS),
            button("List").on_press(Message::LISTS),
            button("Move").on_press(Message::MOVE),
            button("Select").on_press(Message::SELECT),
            
        ];

        
        let mut stringy = String::new();

        self.selected.iter().for_each(|pb| stringy.push_str(pb.to_str().unwrap()));

        
        match self.format {
            true => todo!(),
            false => scrollable(column![a,b,text(stringy),row(vec_of_data).spacing(10)]).width(800).into(),
        }   
    }
}