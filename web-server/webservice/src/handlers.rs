use crate::{models::LoginData};
use log::{info, error};
use super::db_access::*;

use super::state::AppState;
use actix_web::{web, HttpResponse};
use serde_json;

pub async fn health_check_handler(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    
    *visit_count += 1;
    let response = format!("{} {} times", health_check_response, visit_count);
    
    info!("Tested health: {}", visit_count);
    HttpResponse::Ok().body(response) // <- send response
}

use super::models::Animal;
// use chrono::Utc;

// async fn animal_len(app_state: web::Data<AppState>) -> usize {
//     app_state
//     .animals
//     .lock()
//     .unwrap()
//     .clone()
//     .into_iter()
//     .collect::<Vec<Animal>>()
//     .len()
// }


/** 
 * http://.../login
 * 
 * 登陆请求
 * 
 * 传入: json 
 * 
 * 内存实现:   true
 * 数据库实现: false
 * 
*/
pub async fn post_login_process(
    app_state: web::Data<AppState>,
    new_login_request: web::Form<LoginData>,  //如果使用form提交
    // new_login_request: web::Json<LoginData>,  //如果使用json提交
) -> HttpResponse {
    info!("Received login request");
    let mut login_data = app_state.login_data.lock().unwrap();
    login_data.push(LoginData {
        username: new_login_request.clone().username, 
        password: new_login_request.clone().password
    });
    HttpResponse::Ok().body(format!("Login success: {} {}", new_login_request.username, new_login_request.password)) // <- send response
}


pub async fn get_login_data(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    info!("Received login data list");
    let mut string = String::new();
    for data in app_state.login_data.lock().unwrap().iter() {
        string.push_str(&format!("{:?}, ", data));
        // info!("{:?}", data);
    }
    info!("{}", string);
    HttpResponse::Ok().body(format!("{:?}", app_state.login_data.lock().unwrap().clone()))
    
}

/** 
 * http://.../animals/{id}
 * 
 * 添加新动物 
 * 
 * 传入: json 
 * 
 * 内存实现:   true
 * 数据库实现: true
 * 
*/
pub async fn post_new_animal(
    app_state: web::Data<AppState>,
    new_animal: web::Json<Animal>, 
) -> HttpResponse {
    info!("Received new animal");

    /* --- 内存实现 --- */
    // let animal_count = app_state
    //     .animals
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|animal| animal.teacher_id == new_animal.teacher_id)
    //     .collect::<Vec<Animal>>()
    //     .len();

    // let new_animal = Animal {
    //     teacher_id: new_animal.teacher_id,
    //     id: Some(animal_count + 1),
    //     name: new_animal.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };
    // app_state.animals.lock().unwrap().push(new_animal);
    // HttpResponse::Ok().json("Animal added")
    /* === 内存实现 === */

    /* --- 数据库实现 --- */
    let animal = post_new_animal_db(
        &app_state.db,
        new_animal.into()
    ).await;
    /* === 数据库实现 === */

    HttpResponse::Ok().json(animal)
}


/** 
 * http://.../animals/
 * 
 * 从id获取动物们（vec
 * 
 * 内存实现:   true
 * 数据库实现: false
 * 
*/
// use super::models::RequestId;
// 旧名 // pub async fn list_animal(request_id: web::Json<RequestId>, app_state: web::Data<AppState>) -> HttpResponse {
pub async fn get_animals(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    info!("List animals");

    /* --- 内存实现 --- */
    // let animal_items: Vec<Animal> = app_state.animals.lock().unwrap().iter().map(|animal| {
    //     Animal {
    //         teacher_id: animal.teacher_id,
    //         id: animal.id,
    //         name: animal.name.clone(),
    //         time: animal.time,
    //     }
    // }).collect();
    // let animal_item = app_state
    //     .animals
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|x| x.id == Some(request_id.id))
    //     .clone()
    //     .unwrap()
    //     .to_string();
    /* === 内存实现 === */
        
    // HttpResponse::Ok().json(serde_json::to_string(&animal_items).unwrap())


    HttpResponse::Ok().json("Success but nothing happened")
}


/** 
 * http://.../animals/{id}
 * 
 * 从id获取动物们（vec
 * 
 * 内存实现:   false
 * 数据库实现: true
 * 
*/
pub async fn get_animal_by_id(
    app_state: web::Data<AppState>, 
    params: web::Path<(i32,)> 
) -> HttpResponse {
    info!("Get animal by id: {}", params.0);

    /* --- 数据库实现 --- */
    let animal_id = i32::try_from(params.0).unwrap();
    let animals = get_animals_by_id_db(
        &app_state.db, 
        animal_id,
    ).await;
    /* === 数据库实现 === */

    HttpResponse::Ok().json(animals)
}


// pub async fn login(login_data: web::Json<LoginData>, app_state: web::Data<AppState>) -> HttpResponse {
//     println!("Received login request");
//     let user_count = app_state
//         .login_data
//         .clone()
//         .into_iter()
//         .filter(|user| user.username == login_data.username && user.password == login_data.password)
//         .collect::<Vec<LoginData>>()
//         .len();

//     if user_count == 1 {
//         HttpResponse::Ok().json("Login successful")
//     } 
//     else {
//         HttpResponse::Unauthorized().json("Login failed")
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    
    use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[actix_rt::test]
    async fn post_animal_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL invalid");
        let db_pool = PgPoolOptions::new()
            // .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();

        let animal = web::Json(Animal {
            id: 1,
            name: "David".into(),
            time: None,
        });
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
            login_data: Mutex::new(vec![LoginData {
                username: "".to_string(),
                password: "".to_string(),
            }])
        });
        let resp = post_new_animal(app_state, animal).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_animals_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL invalid");
        let db_pool = PgPoolOptions::new()
            // .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
            login_data: Mutex::new(vec![LoginData {
                username: "".to_string(),
                password: "".to_string(),
            }])
        });
        let resp = get_animals(app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_animal_by_id_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL invalid");
        let db_pool = PgPoolOptions::new()
            // .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
            login_data: Mutex::new(vec![LoginData {
                username: "".to_string(),
                password: "".to_string(),
            }])
        });
        let animal_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_animal_by_id(app_state, animal_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}