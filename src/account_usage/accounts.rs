#[derive(Debug)]
pub struct User{
    name: String,
    password: String,
    id: i64,
}

pub struct Accounts{
    users: Vec<User>,
    pub logged_in_user: i64,
}

pub fn new() -> Accounts{
    return Accounts { users: vec![], logged_in_user: 0 };
} 

impl Accounts{
    pub fn create_new(&mut self,username: &String, password: &String) -> bool{
        if self.is_user_exists(username) {
            return false
        };
        let  index = self.users.len() as i64;
        self.users.push(User {
            id: index + 1,
            password: password.clone(),
            name: username.clone(),
        });
        return true;
    }
    pub fn is_user_exists(&self, user_name: &String) -> bool{
        return self.users.iter().find(|u| &u.name == user_name).is_some();
    }
    
    pub fn delete_user(&mut self) -> bool {
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

    pub fn login(&mut self, username: String, password: String) -> bool {
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

    pub fn logout(&mut self)  {
        self.logged_in_user = 0;
    }

    pub fn get_user_by_name(&self, name: &String) -> Option<&User> {
        return self.users.iter().find(|u| &u.name == name)
    }

}