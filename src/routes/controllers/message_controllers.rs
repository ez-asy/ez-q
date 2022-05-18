use actix_web::{post, web, HttpResponse, Responder};
use fred::pool::RedisPool;
use serde::{Deserialize, Serialize};

use crate::core::{
    add_message_to_complete::add_message_to_complete_set,
    add_message_to_failed::add_message_to_failed_set,
    remove_message_from_processing::remove_message_from_processing_set,
};

#[derive(Deserialize)]
pub struct MessagePath {
    msg_id: String,
}

#[derive(Serialize)]
pub struct FinishMessageResponse {
    id: String,
}

#[post("/messages/{msg_id}/complete/")]
pub async fn complete_message(
    redis_pool: web::Data<RedisPool>,
    path: web::Path<MessagePath>,
) -> impl Responder {
    // Get client from pool
    let redis_client = redis_pool.next();

    println!("{:?}", &path.msg_id);
    // Get vec of queue names
    let is_message_removed = match remove_message_from_processing_set(redis_client, &path.msg_id)
        .await
    {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Error attempting to complete message")
        }
    };

    if !is_message_removed {
        return HttpResponse::NotFound().body(format!(
            "Message {} not completed. A message of this id is not being processed.",
            path.msg_id
        ));
    }

    let _is_message_added_to_complete =
        match add_message_to_complete_set(redis_client, &path.msg_id).await {
            Ok(value) => value,
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .body("Error attempting to complete message")
            }
        };

    let response = FinishMessageResponse {
        id: (&path.msg_id).to_owned(),
    };

    HttpResponse::Ok().json(response)
}

#[post("/messages/{msg_id}/fail/")]
pub async fn fail_message(
    redis_pool: web::Data<RedisPool>,
    path: web::Path<MessagePath>,
) -> impl Responder {
    // Get client from pool
    let redis_client = redis_pool.next();

    // Get vec of queue names
    let is_message_removed =
        match remove_message_from_processing_set(redis_client, &path.msg_id).await {
            Ok(value) => value,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Error attempting to fail message")
            }
        };

    if !is_message_removed {
        return HttpResponse::NotFound().body(format!(
            "Message {} not fail. A message of this id is not being processed.",
            path.msg_id
        ));
    }

    let _is_message_added_to_complete =
        match add_message_to_failed_set(redis_client, &path.msg_id).await {
            Ok(value) => value,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Error attempting to fail message")
            }
        };
    let response = FinishMessageResponse {
        id: (&path.msg_id).to_owned(),
    };

    HttpResponse::Ok().json(response)
}
