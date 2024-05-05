use iced::{widget, Alignment, Element, Font, Pixels, Theme};
use iced::widget::{button, column, text,text_input,Space,image, row, scrollable, space,Button};
use iced::{Application, Command, Settings, executor, window};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use pad::PadStr;
use orion::aead;
use crate::users::User;
// use crate::file_explorer;

use iced::widget::image::Handle;
// use std::fs::{self, read, read_dir, File, OpenOptions};
use std::{fmt,  vec};
use std::cmp;
use std::collections::HashSet;
use tokio;
use std::time::Instant;


pub fn main() -> iced::Result {
    let ferry = Some(window::icon::from_file("img/ferry.png").unwrap());
    let settings = Settings {
        window: window::Settings {
            size: iced::Size { width: 600.0f32, height: 300.0f32 },
            resizable: false,
            decorations: true,
            level: window::Level::Normal,
            position: window::Position::Centered,
            icon: ferry,  
            ..Default::default()
        },
        ..Default::default()
    };
    TextBox::run(settings)
} 

#[derive(Debug,Clone)]
pub enum Scene {
    LOGIN,
    REGISTER,
    FILES,
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug,Clone)]
pub enum Message {
    UserName(String),
    Password(String),
    ENTER,
    SCENE(Scene),
    BACK,
    Selected(PathBuf),
    MOVE,
    SUBMIT,
    ENCODE,
    USER,
    TEST,
}

#[derive(Debug)]
pub struct TextBox {
    user: String,
    pass: String,
    error: String,
    scene: Scene,
    name: String,
    secret_key: aead::SecretKey,
    user_base: HashMap<String, User>,
    path: PathBuf,
    dir: Vec<PathBuf>,
    file_png: image::Handle,
    folder_png: image::Handle,
    selected: HashSet<PathBuf>,
    moves: bool,
    user_dir: PathBuf,
    encrypt: bool,
    move_select: String,
    encrypt_decrypt: String,
    update_message: String,
    test_bool: bool,
}

impl Application for TextBox {

    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;


    fn new(_flags: ()) -> (TextBox, Command<Self::Message>) {
        (TextBox {
            user: String::new(),
            pass: String::new(),
            error: String::new(),
            scene: Scene::LOGIN,
            name: String::from("LOGIN"),
            secret_key: aead::SecretKey::default(),
            user_base: User::get_existing(),
            path: PathBuf::new(),
            dir: vec![],
            file_png: Handle::from_path("img/File.png"),
            folder_png: Handle::from_path("img/Folder.png"),
            selected: HashSet::new(),
            moves: false,
            user_dir: PathBuf::new(),
            encrypt: true,
            move_select: String::from("Move"),
            encrypt_decrypt: String::from("Decrypt"),
            update_message: String::new(),
            test_bool: true,
        }, Command::none())
    }

    fn title(&self) -> String {
        self.name.clone()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UserName(a) => self.user = a,
            Message::Password(a) => self.pass = a,
            Message::ENTER => {
                match self.scene {
                    Scene::LOGIN => {
                        match self.user_base.get(&self.user.pad_to_width(32)) {
                            Some(x) => {
                                match self.pass.pad_to_width(32).eq(&x.password) {
                                    true => {
                                        self.user_dir = get_user_dir(&self.user);
                                        self.secret_key = aead::SecretKey::from_slice(&x.secret_password).unwrap();
                                        self.scene = Scene::FILES;
                                        self.path = fs::canonicalize(PathBuf::from("./")).unwrap(); //gets absolute path
                                        self.dir = dir_to_paths(fs::canonicalize(PathBuf::from("./")).unwrap().as_path());
                                        self.error = String::new();
                                        return iced::window::resize(window::Id::MAIN, iced::Size::new(820.0f32,480.0f32));

                                    },
                                    false => self.error = String::from("Username or password is incorrect"),
                                }
                            },
                            None => self.error = String::from("Username or password is incorrect"),
                        }
                    }, 
                    Scene::REGISTER => {
                        if user_password_parameters(self.user.as_bytes(), self.pass.as_bytes()) && !self.user_base.contains_key(&self.user.pad_to_width(32)) {
                            self.user_dir = get_user_dir(&self.user);
                            self.secret_key = aead::SecretKey::default();
                            write_to_info(self.user.clone().into_bytes(), self.pass.clone().into_bytes(), self.secret_key.unprotected_as_bytes()).expect("File Failure");
                            self.scene = Scene::FILES;
                            self.path = fs::canonicalize(PathBuf::from("./")).unwrap(); //gets absolute path
                            self.dir = dir_to_paths(fs::canonicalize(PathBuf::from("./")).unwrap().as_path());
                            self.error = String::new();
                            return iced::window::resize(window::Id::MAIN, iced::Size::new(820.0f32,480.0f32));
                        }
                        else {
                            self.error = user_password_problems(self.user.as_bytes(), self.pass.as_bytes());
                        }
                    }
                    _ => panic!(),
                }
            },
            Message::BACK => {
                match self.path.pop() {
                    false => todo!("maybe a lil screen shake or popup?"),
                    true => self.dir = dir_to_paths(&self.path),
                }
            },
            Message::MOVE => match self.moves {
                true => {
                    self.moves = false;
                    self.move_select = String::from("Move");
                },
                false =>  {
                    self.moves = true;
                    self.move_select = String::from("Select");
                },
            },
            Message::Selected(pb) => {
                if !self.update_message.is_empty() {self.update_message = String::new();}
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
            Message::SUBMIT => {
                self.selected.iter().for_each(|pb|
                    match self.encrypt {
                        true => {
                            match pb.is_file() {
                                true => file_encrypt(pb, &self.secret_key, &self.user_dir).unwrap(),
                                false => {
                                    match self.test_bool {
                                        true => {
                                            let now = Instant::now();
                                            let _ = folder_encrypt_conccur(pb.clone(), &self.secret_key, self.user_dir.clone(),true);
                                            let a = now.elapsed().as_secs_f32();
                                            println!("Parallel took {} seconds", a);

                                        },
                                        false => {
                                            let now = Instant::now();
                                            folder_encrypt(pb, &self.secret_key, &self.user_dir,true).unwrap();
                                            let b = now.elapsed().as_secs_f32();
                                            println!("Regular took {} seconds", b);

                                        },
                                    }
                                },
                            };
                        },
                        false => {
                            match pb.is_file() {
                                true => match file_decrypt(pb, &self.secret_key, &self.user_dir) {
                                    Err(_) => self.error = String::from("Unable to Decrypt, wrong key"),
                                    Ok(_) => (),
                                },
                                false => {
                                    match self.test_bool {
                                        true => {
                                            let now = Instant::now();
                                            let _ = folder_encrypt_conccur(pb.clone(), &self.secret_key, self.user_dir.clone(),false);
                                            let a = now.elapsed().as_secs_f32();
                                            println!("Parallel took {} seconds", a);

                                        },
                                        false => {
                                            let now = Instant::now();
                                            match folder_encrypt(pb, &self.secret_key, &self.user_dir,false) {
                                                Err(_) => self.error = String::from("Unable to decrypt, wrong key"),
                                                Ok(_) => (),
                                            }
                                            let b = now.elapsed().as_secs_f32();
                                            println!("Regular took {} seconds", b);

                                        },
                                    }
                                },

                                false => match folder_encrypt(pb, &self.secret_key, &self.user_dir,false) {
                                    Err(_) => self.error = String::from("Unable to Decrypt, wrong key"),
                                    Ok(_) => (),
                                },
                            }
                        
                        },
                    }
                );
                self.update_message = String::from("Files submitted to user file!");
                self.selected.clear();
                self.dir = dir_to_paths(&self.path);
            },
            Message::SCENE(s) => {
                self.scene = s;
                self.name = self.scene.to_string();
            },
            Message::ENCODE => match self.encrypt {
                true => {
                    self.encrypt = false;
                    self.encrypt_decrypt = String::from("Encrypt");
                },
                false =>  {
                    self.encrypt = true;
                    self.encrypt_decrypt = String::from("Decrypt");
                },
            },
            Message::USER => {
                self.path = fs::canonicalize(self.user_dir.clone()).unwrap();
                self.dir = dir_to_paths(&self.path);
            },
            Message::TEST => {
                if self.test_bool {
                    self.test_bool = false;
                }
                else {
                    self.test_bool = true;
                }
            }
        }
        Command::none()
    }

    fn theme(&self) -> Theme {
        widget::theme::Theme::Dracula
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.scene {
            Scene::LOGIN => {
                let user = text_input("Empty", &self.user,)
                .on_input(Message::UserName)
                .on_submit(Message::ENTER)
                .padding(10)
                .size(20);
            
                let pass = text_input("", &self.pass,)
                .on_input(Message::Password)
                .on_submit(Message::ENTER)
                .padding(10)
                .size(20)
                .secure(true)
                .icon(text_input::Icon { 
                    font: Font::default(), 
                    code_point: '🔒', 
                    size: Some(Pixels(28.0)), 
                    spacing: 10.0, side: 
                    text_input::Side::Right,
                });
                
                let a = column![
                    text("Username").size(18),
                    user,
                    text("Password").size(18),
                    pass,
                    Space::new(0, 10),
                    button("Confirm").on_press(Message::ENTER),
                    button("Not a user? Sign up").on_press(Message::SCENE(Scene::REGISTER)),
                    Space::new(0,10),
                    text(format!("{}",self.error)).size(18),
                ]
                .padding(10)
                .align_items(Alignment::Start);
            
                a.into()                
            },
            Scene::REGISTER => {
                let user = text_input("Empty", &self.user,)
                .on_input(Message::UserName)
                .on_submit(Message::ENTER)
                .padding(10)
                .size(20);
            
                let pass = text_input("", &self.pass,)
                .on_input(Message::Password)
                .on_submit(Message::ENTER)
                .padding(10)
                .size(20)
                // .secure(true)
                .icon(text_input::Icon { 
                    font: Font::default(), 
                    code_point: '🔒', 
                    size: Some(Pixels(28.0)), 
                    spacing: 10.0, side: 
                    text_input::Side::Right,
                });
                
                let a = column![
                    text("Username").size(18),
                    user,
                    text("Password").size(18),
                    pass,
                    Space::new(0, 10),
                    button("Confirm").on_press(Message::ENTER),
                    button("Are a user? Log in").on_press(Message::SCENE(Scene::LOGIN)),
                    Space::new(0,10),
                    text(format!("{}",self.error)).size(18),
                    // text(format!("Password: {} ",self.final_password)).size(18),
                    // text(format!("Username: {} ",self.final_username)).size(18),

                ]
                .padding(10)
                .align_items(Alignment::Start);
            
                a.into()
            },
            Scene::FILES => {

                // let window_size = iced::Size::ZERO;
                // let _ = iced::window::fetch_size(window::Id::MAIN, move |x: iced::Size| x);
                // println!("{}", window_size.width as u32 / 80);
                // let number_of_columns = cmp::max(10,window_size.width as usize / 80);

                let number_of_columns = 10;

                let mut vec_of_data: Vec<Element<'_, Self::Message>> = Vec::with_capacity(number_of_columns);

                let a = text(self.path.to_str().unwrap()).size(18);


                for column_number in 0..cmp::min(number_of_columns, self.dir.len()) { //Runs at MAX  times
                    let mut c: Vec<Element<'_, Self::Message>> = vec![];

                    let mut leftover = 0;

                    if column_number == 0 {
                        vec_of_data.push(space::Space::with_width(5).into());
                    }

                    if self.dir.len() % number_of_columns >= column_number {
                        leftover = 1;
                    }

                    for row_number in 0..(self.dir.len()/number_of_columns) + leftover {
                        
                        // println!("{row_number}: | {number_of_columns}: | {column_number}: |{}",(row_number * (number_of_columns - 1)) + column_number)
                        // println!("i: {}| j: {}| j*7 +i: {}",column_number,row_number,(row_number * (number_of_columns-1)) + column_number)
                        let x = &self.dir[(row_number * (number_of_columns - 1)) + column_number];

                        match x.is_file() {
                            true => {
                                // println!("FILE:\t{}",x.file_name().unwrap().to_str().unwrap());                         
                                c.push(
                                    column![
                                        Button::new(x.file_name().unwrap().to_str().unwrap().get(..cmp::min(7,x.file_name().unwrap().to_str().unwrap().len())).unwrap())
                                        .width(70)
                                        .height(35)
                                        .clip(false)
                                        .on_press(Message::Selected(x.to_path_buf())),
                                        image(self.file_png.clone()).width(70).height(70),
                                    ]
                                    .align_items(iced::Alignment::Center)
                                    .into()
                                )
                            },
                            false => {
                                // println!("FOLDER\t{}",x.file_name().unwrap().to_str().unwrap());                         
                                c.push(
                                column![
                                    Button::new(x.file_name().unwrap().to_str().unwrap().get(..cmp::min(7, x.file_name().unwrap().to_str().unwrap().len())).unwrap())
                                    .width(70)
                                    .height(35)
                                    .clip(false)
                                    .on_press(Message::Selected(x.to_path_buf())),
                                    image(self.folder_png.clone()).width(70).height(70),
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
                    button(&self.move_select[..]).on_press(Message::MOVE),
                    button("SUBMIT").on_press(Message::SUBMIT),
                    button(&self.encrypt_decrypt[..]).on_press(Message::ENCODE),
                    button("go to user folder").on_press(Message::USER),
                    button("SPEED TESTER").on_press(Message::TEST),
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
                    text(&self.update_message).size(18),
                    err,
                    b,
                    column(selected_elements).spacing(10),
                    space::Space::with_height(20),row(vec_of_data).spacing(10)])
                    .width(820).into()
            },
        }
        
    }    
}


// -----------------------------------------------------------------------------------------------------------
//functions related to the Files scene

//UNUSED
//will print all paths from a string
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

//UNUSED
//will convert a path to Vector of Strings
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

//will conver a path to Vector of Strings
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

//UNUSED
//makes a user directory
fn _make_dir_if_needed(path: &str) -> () {
    // Check if the directory exists
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_dir() {
            println!("Found directory!.");
        } else {
            println!("A file with the same name exists, not a directory.");
        }
    } else {
        println!("No users. Creating...");
        //let users = "./users";
        match fs::create_dir(path){
            Ok(_) => println!("Made directory {}", path),
            Err(_err) => println!("Fcuk this shit"),
        }
    }
}

//returns the user dir, makes one if none exists
fn get_user_dir(user: &str) -> PathBuf {
    if !PathBuf::from("./users").exists() {
        let _ = fs::create_dir("./users");
    };

    let path = PathBuf::from(format!("./users/{}", user));

    match path.exists() {
        true => path.to_path_buf(),
        false => {
            println!("Created new directory");
            let _ = fs::create_dir(format!("./users/{}", user));
            PathBuf::from(format!("./users/{}", user))
        },
    }
}

//reads the file contents and sends to write file to encrypt
pub fn file_encrypt(file: &Path, secret_key: &aead::SecretKey, dir: &Path) -> Result<(), Box<dyn Error>> {
    let information: Vec<u8> = fs::read(file)?;
    let sealed = aead::seal(secret_key, &information).expect("Open problem");
    match write_to_file(&sealed, &file,true, dir) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
}

//reads the file contents and sends to write file to encrypt
fn file_decrypt(file: &Path, secret_key: &aead::SecretKey, dir: &Path) -> Result<(), Box<dyn Error>> {
    let information: Vec<u8> = fs::read(file)?;
    let opened = aead::open(secret_key, &information)?;
    match write_to_file(&opened, &file,false, dir) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
}


//will make a new file that is encrypted or decrypted
fn write_to_file(s: &[u8], file: &Path, encrypt: bool, dir: &Path) ->  Result<File, std::io::Error> {
    
    let file_type = match file.extension() {
        Some(x) => format!(".{}",x.to_str().unwrap()),
        None => String::new(),
    };

    let file_name = file.file_stem().unwrap().to_str().unwrap();

    let e_or_d = match encrypt {
        true => "_e",
        false => "_d",
    };

    let mut temp = format!("{}/{}{}{}",dir.to_str().unwrap(),file_name,e_or_d,file_type);

    let mut i = 0;
    while Path::new(&temp).exists() {
        temp = format!("{}/{}{}{}{}",dir.to_str().unwrap(),i.to_string(), file_name,e_or_d,file_type);
        i += 1;
    }

    let p = Path::new(&temp);
    let mut file = File::create(p)?;
    file.write_all(s)?;
    Ok(file)
}

//writes to file that keeps user info
fn write_to_info(mut u: Vec<u8>,mut p: Vec<u8>, s: &[u8]) ->  Result<File, std::io::Error> {

    if !PathBuf::from("./info").exists() {
        let _ = fs::create_dir("./info");
    };

    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("info/info.txt")
            .unwrap();

    u.resize(32,32); //make the username and password take up 32 bytes
    p.resize(32, 32);
    
    file.write_all(&u)?;
    file.write_all(&p)?;
    file.write_all(s)?;
    write!(file,"\n")?;

    Ok(file)
}

//Things need to return true,
//Username is between length 4 and 32
//Password is between length 8 and 32
//Username must be unique
//Possibly more in future
fn user_password_parameters(username: &[u8], password: &[u8]) -> bool {
    username.len() >= 4 &&  username.len() <= 32 && password.len() <= 32 && password.len() >= 8
}

//returns the problem
fn user_password_problems(username: &[u8], password: &[u8]) -> String {
    let mut problem = String::new();
    if username.len() < 4 || username.len() > 32 {
        problem = problem + "Username needs to be between 4 and 32 characters\n";
    }
    else if password.len() < 8 || password.len() > 32 {
        problem = problem + "Password needs to be between 8 and 32 characters\n";
    }
    else {
        problem = problem + "Username is not unique\n";
    }

    problem
}

//Make a function that will output a rust executable file that will encrypt and decrypt files from command line

//default folder encrypter
fn folder_encrypt(data_path: &Path, secret_key: &aead::SecretKey, user_path: &Path, encrypt: bool) -> Result<(), Box<dyn Error>>{
    match data_path.is_dir() { //match path points to folder
        true => {
            let new_path = match encrypt {
                true => format!("{}/{}_e",
                    user_path.to_str().unwrap(), data_path.file_name().unwrap().to_str().unwrap()),
                false => format!("{}/{}_d",
                    user_path.to_str().unwrap(), data_path.file_name().unwrap().to_str().unwrap()),
            };

            let _ = fs::create_dir(PathBuf::from(new_path.clone()));

            dir_to_paths(data_path).iter().for_each(
                |dp| 
                folder_encrypt(&dp, secret_key, &PathBuf::from(&new_path), encrypt).unwrap());
        },
        false => {
            match encrypt {
                true => file_encrypt(data_path, secret_key, user_path)?,
                false => file_decrypt(data_path, secret_key, user_path)?,
            }
        },
    };
    Ok(())
} 

//attempt to make a folder encrypter with conccurency
#[tokio::main]
async fn folder_encrypt_conccur(data_path: PathBuf, secret_key: &aead::SecretKey, user_path: PathBuf, encrypt: bool) {
    let mut stk: Vec<(PathBuf, PathBuf)> = vec![(data_path, user_path)];
    while !stk.is_empty() {
        let current = stk.pop().unwrap();
        match current.0.is_dir() { //match path points to folder
            true => {
                let new_path = match encrypt {
                    true => format!("{}/{}_e",
                        current.1.to_str().unwrap(), current.0.file_name().unwrap().to_str().unwrap()),
                    false => format!("{}/{}_d",
                        current.1.to_str().unwrap(), current.0.file_name().unwrap().to_str().unwrap()),
                };
    
                let _ = fs::create_dir(PathBuf::from(new_path.clone()));
    
                let p = PathBuf::from(new_path);

                let mut combine: Vec<(PathBuf, PathBuf)> = dir_to_paths(&current.0).iter().map(|x| (x.clone(),p.clone())).collect();

                stk.append(&mut combine);
            },
            false => {
                tokio::spawn(a_file_encrypt_decrypt(current.0, secret_key.unprotected_as_bytes().to_vec(), current.1, encrypt));
            },
        };
    
    }

}


async fn a_file_encrypt_decrypt(file: PathBuf, secret_key: Vec<u8>, dir: PathBuf, encrypt: bool) {
    let information: Vec<u8> = fs::read(&file).expect("fail");
    let sec = aead::SecretKey::from_slice(&secret_key).unwrap();

    let sealed = match encrypt {
        true => aead::seal(&sec, &information).expect(&format!("Open problem for file {}", file.to_str().unwrap())),
        false => aead::open(&sec, &information).expect("YOU TRIED TO OPEN AN ENCYRPTION THAT WASNT YOURS"),
    };

    write_to_file(&sealed, &file,true, &dir).expect("FAIL");
}

