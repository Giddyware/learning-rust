use std::{collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};

pub fn hash_password(password:&str) ->  String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn greet_user(name:&str) -> String {
    format!("Hello {name}")
}

//function that reads Input
pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect( "Stdin not working");
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]

pub enum LoginAction {
    Granted(LoginRole),
    Denied
}


#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]

pub enum LoginRole {
    Admin,
    User,
   
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}


// Constructor
impl User {
    pub fn new(username: &str, password:&str, role: LoginRole) -> User{
        Self { username: username.to_lowercase(), password: hash_password(password), role }
    }
 }

//  Static arrays function implementation of the get user

//  pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("adufe", "password", LoginRole::User)
//     ]
//  }

//  fn get_admin_users(){
 
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|u | u.role == LoginRole::Admin)
//         .map(|u|u.username)
//         .collect();
//  }

fn get_default_users () -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("adufe".to_string(), User::new("adufe", "password", LoginRole::User));
    users
}

fn get_users () -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).expect("Could not read users file");
        let users: HashMap<String, User> =  serde_json::from_str(&users_json).expect("Could not parse users file");
        users
    } else {
        // Create a file and return the default users
        let users=  get_default_users();
        let users_json = serde_json::to_string(&users).expect("Could not serialize users");
        std::fs::write(users_path, users_json).expect("Could not write users file");
        users
    } 
}



pub fn login(username:&str,password:&str) -> Option<LoginAction>{
    let username = username.to_lowercase();
    let password = hash_password(password);

    let users = get_users();

    if let Some(user) = users.get(&username){
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }


    // if let Some(user) = users.iter().find(|user |user.username == username){
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     } else {
    //         return Some(LoginAction::Denied);
    //     }
    // }
    None
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user(){
        assert_eq!("Hello Adufe", greet_user("Adufe"));
    }

    #[test]
    fn test_login(){
        // assert_eq!(login("admin","password"), LoginAction::Admin);
        // assert_eq!(login("adufe","password"), LoginAction::User);
        // assert_eq!(login("admin-not","password"), LoginAction::Denied);
        
    }
  
}
