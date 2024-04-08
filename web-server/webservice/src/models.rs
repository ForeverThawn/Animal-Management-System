use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Animal {
    pub id: i32,
    // pub animal_id: Option<usize>,  // 可空
    pub name: String,
    pub time: Option<NaiveDateTime>,  // 可空
}

impl From<web::Json<Animal>> for Animal {
    fn from(animal: web::Json<Animal>) -> Self {
        Animal {
            id: animal.id,
            name: animal.name.clone(),
            time: animal.time,
            
        }
    }
}

impl ToString for Animal {
    // to_json_string
    fn to_string(&self) -> String {
        let id = &self.id;
        let name = &self.name;

        format!(
            "{{\"id\": {}, \"name\": \"{}\", \"time\": \"{}\"}}", 
            id,
            name,
            "None"
        )
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl From<web::Json<LoginData>> for LoginData {
    fn from(value: web::Json<LoginData>) -> Self {
        LoginData {
            username: value.username.clone(),
            password: value.password.clone(),
        
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestId {
    pub id: usize,
}