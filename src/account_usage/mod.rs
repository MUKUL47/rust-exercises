use std::{*, process::exit, io::Write};
mod accounts;
pub fn start(){
    let mut accounts:accounts::Accounts = accounts::new();
    let mut input_str: String = String::from("");
        option_menu(&accounts);
        loop {
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
                option_menu(&accounts);
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
fn option_menu(accounts: &accounts::Accounts){
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