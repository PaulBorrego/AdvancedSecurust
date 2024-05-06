use std::{collections::HashMap, fs::{self, OpenOptions}, io::Read, path::PathBuf};

#[derive(PartialEq, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub secret_password: Vec<u8>,
}
impl User {
    pub fn get_existing() -> HashMap<String,User> {

        if !PathBuf::from("./info").exists() {
            let _ = fs::create_dir("./info");
        };
        
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("info/info.txt")
            .unwrap();


        let mut buffer = Vec::new();
        let a = f.read_to_end(&mut buffer).expect("Reading Problem") / 97;
        let mut returning_users: HashMap<String, User> = HashMap::new();
        for i in 0..a {
            let b = i * 97;
            returning_users.insert(
                String::from_utf8(buffer[b..b+32].to_vec()).unwrap(),
                User {
                    username: String::from_utf8(buffer[b..b+32].to_vec()).unwrap(),        
                    password: String::from_utf8(buffer[b+32..b+64].to_vec()).unwrap(),
                    secret_password: buffer[b+64..b+96].to_vec(),
    
                }
            );
        }
        returning_users
    }
   
}