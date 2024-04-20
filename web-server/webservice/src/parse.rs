// use super::models::LoginData;
// use serde::Serialize;
// use actix_web::web;

// pub fn parse_json_from_form( 
//     form: web::Form<LoginData>,
// ) -> web::Json<LoginData> {
//     web::Json(LoginData {
//         username: form.username.clone(),
//         password: form.password.clone(),
//     })
// }