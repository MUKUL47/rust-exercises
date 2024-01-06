#[derive(Debug)]
struct User{
    name: String,
    password: String,
    id: i64,
}

struct Accounts{
    users: Vec<User>,
    logged_in_user: i64,
}

impl Accounts{
    pub fn create_new(&mut self,user: User) -> bool{
        if self.is_user_exists(&user.name) {
            return false
        };
        let  index = self.users.len() as i64;
        self.users.push(User {
            id: index + 1,
            password: String::from(user.password),
            name: String::from(user.name),
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

}