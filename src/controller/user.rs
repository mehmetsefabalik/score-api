use actix_web::{http, web, Error, HttpResponse};
use futures::future::Future;
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
  display_name: String,
  country: String
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
  user_id: bson::Bson,
}

#[derive(Deserialize)]
pub struct SubmitScoreRequest {
  score: i64,
}

pub fn create(body: web::Json<CreateUserRequest>, pool: web::Data<Pool<MongodbConnectionManager>>) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || crate::service::user::create(&body.display_name, &body.country, pool)).then(|_result| match _result {
    Ok(inserted_result) => {
      match inserted_result.inserted_id {
        Some(id) => HttpResponse::Ok().json(CreateUserResponse {
          user_id: id,
        }),
        None => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
      }
    },
    Err(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  })
}

pub fn get(path: web::Path<(String)>, pool: web::Data<Pool<MongodbConnectionManager>>) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || crate::service::user::get(&path, pool)).then(|_result| match _result {
    Ok(get_result) => {
      match get_result {
        Some(result) => HttpResponse::Ok().json(result),
        None => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
      }
    },
    Err(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  })
}

pub fn submit_score(path: web::Path<(String)>, body: web::Json<SubmitScoreRequest>, pool: web::Data<Pool<MongodbConnectionManager>>) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || crate::service::user::increment_score(&path, body.score, pool)).then(|_result| match _result {
    Ok(get_result) => {
      match get_result {
        Some(result) => HttpResponse::Ok().json(result),
        None => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
      }
    },
    Err(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  })
}
