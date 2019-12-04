use actix_web::web;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, coll, doc};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;

pub fn create(
  display_name: &str,
  country: &str,
  pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<mongodb::coll::results::InsertOneResult, mongodb::error::Error> {
  pool
    .get()
    .expect("can not get pool")
    .collection("users")
    .insert_one(
      doc! {"display_name" => display_name, "country" => country},
      None,
    )
}

pub fn get(
  user_id: &str,
  pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<std::option::Option<bson::ordered::OrderedDocument>, mongodb::error::Error> {
  pool
    .get()
    .expect("can not get pool")
    .collection("users")
    .find_one(
      Some(doc! {"_id" => mongodb::oid::ObjectId::with_string(user_id).unwrap()}),
      None,
    )
}

pub fn increment_score(
  user_id: &str,
  score: i64,
  pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<std::option::Option<bson::ordered::OrderedDocument>, mongodb::error::Error> {
  pool
    .get()
    .expect("can not get pool")
    .collection("users")
    .find_one_and_update(
      doc! {"_id" => mongodb::oid::ObjectId::with_string(user_id).unwrap()},
      doc! {"$inc" => doc! {"score_worth" => score}},
      Some(coll::options::FindOneAndUpdateOptions {
        return_document: Some(coll::options::ReturnDocument::After),
        max_time_ms: None,
        projection: None,
        sort: None,
        upsert: Some(true),
        write_concern: None,
      }),
    )
}
