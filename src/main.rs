use std::{*, process::exit, io::Write};
#[derive(Debug)]
struct User{
    name: String,
    password: String,
    id: i64,
}
#[derive(Debug)]
struct Accounts{
    users: Vec<User>,
    logged_in_user: i64,
}

fn main(){
    let mut accounts:Accounts = Accounts::new();
    let mut is_active_input : bool= true;
    let mut input_str: String = String::from("");
    let mut user_id_init: i64 = 0;
        optionsMenu(&accounts);
        while is_active_input {
        input_str = String::from("");
        std::io::stdin().read_line(&mut input_str).unwrap();
        input_str = input_str.trim_end().to_string();
        let option_number = input_str.trim().parse::<i8>().unwrap();
        match option_number{
            1 => {
                let (tmp_username, tmp_password) = ask_user_data();
                let response = accounts.create_new(&tmp_username, &tmp_password);
                if !response {
                    println!("User already exists!");
                }else{
                    println!("{:?}",accounts);
                    match accounts.get_user_by_name(&tmp_username){
                        Some( user) => {
                            println!("User created successfully! {:?}", user);
                        }
                        None => {

                        }
                    }
                }
            }
            2 => {
                let (tmp_username, tmp_password) = ask_user_data();
                match accounts.login(tmp_username, tmp_password){
                    true => println!("Logged in successfully!"),
                    false =>  println!("User not found !!!")
                }
            }
            4 => {
                optionsMenu(&accounts);
            }
            5 => {
                match accounts.delete_user(){
                    true => println!("Your account deleted successfully"),
                    false =>  println!("Login first !!!")
                }
            }
            6 => {
                accounts.logout();
                println!("Logged out successfully {}",accounts.logged_in_user);
            }
            _ =>{}
        }

    }
}
fn ask_user_data() -> (String, String){
    let mut tmp_username = String::from("");
    let mut tmp_password = String::from("");
    println!("Enter your name");
    std::io::stdin().read_line(&mut tmp_username).unwrap();
    println!("Enter password");
    std::io::stdin().read_line(&mut tmp_password).unwrap();
    tmp_username = tmp_username.trim_end().to_string();
    tmp_password = tmp_password.trim_end().to_string();
    return (tmp_username, tmp_password)
}
fn optionsMenu(accounts: &Accounts){
    println!("*****************");
    if accounts.logged_in_user> 0 {
        println!("!! Welcome user {:?} !!",accounts.logged_in_user);
        println!("4 -> Options ");
        println!("5 -> Delete your account ");
        println!("6 -> Logout ");
    }else{
        println!("1 -> Create User ");
        println!("2 -> Login ");
        println!("3 -> Exit ");
        println!("4 -> Options ");
    }
    println!("*****************");
}
impl User {
    fn new(name: String, password: String, id: i64) -> User{
        User { name: name, password, id }
    }
}
impl Accounts{
    pub fn new () -> Self {
        Accounts {
            logged_in_user: 0,
            users: vec![]
        }
    }
    pub fn create_new(&mut self,username: &String, password: &String) -> bool{
        if self.is_user_exists(username) {
            return false
        };
        let  index = self.users.len() as i64;
        self.users.push(User {
            id: index + 1,
            password: String::from(password),
            name: String::from(username),
        });
        return true;
    }
    fn is_user_exists(&self, user_name: &String) -> bool{
        return self.users.iter().find(|u| &u.name == user_name).is_some();
    }
    
    fn delete_user(&mut self) -> bool {
        if self.logged_in_user == 0 {
            return false;
        }
        for i in 0..self.users.len(){
            if self.users.get(i).unwrap().id == self.logged_in_user {
                self.users.remove(i);
                return true;
            }
        }
        return false;
    }

    fn login(&mut self, username: String, password: String) -> bool {
        match self.users.iter().find(|u| u.name == username && u.password == password){
            Some(user) => {
                self.logged_in_user = user.id;
                return true;
            }
            None => {
                return false;
            }
        }
    }

    fn logout(&mut self)  {
        self.logged_in_user = 0;
    }

    fn get_user_by_name(&self, name: &String) -> Option<&User> {
        return self.users.iter().find(|u| &u.name == name)
    }

}