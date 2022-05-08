use actix_web::{get, post, web, HttpResponse, Responder};
use fred::pool::StaticRedisPool;
use serde::{Deserialize, Serialize};

use crate::core::{
    add_message_to_processing::add_message_to_processing_set,
    add_message_to_queue::add_message_to_queue, get_message_content::get_message_content,
    get_queue_names::get_queue_names, message::Message,
    pop_message_from_queue::pop_message_from_queue, register_new_queue::register_queue,
    save_message::save_message,
};

#[derive(Deserialize)]
pub struct QueuePath {
    queue_name: String,
}

#[derive(Deserialize)]
pub struct PostMessageBody {
    content: serde_json::Value,
}

#[derive(Serialize)]
pub struct ListQueuesResponse {
    queues: Vec<String>,
}

#[derive(Serialize)]
pub struct PublishMessageResponse {
    id: String,
}

#[derive(Serialize)]
pub struct MessageContentResponse {
    id: String,
    content: serde_json::Value,
}

#[get("/queues/")]
pub async fn list_all_queues(redis_pool: web::Data<StaticRedisPool>) -> impl Responder {
    // Get client from pool
    let redis_client = redis_pool.next();

    // Get vec of queue names
    let queue_names = match get_queue_names(redis_client).await {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}", err.to_string());
            return HttpResponse::InternalServerError().body("");
        }
    };

    let response = ListQueuesResponse {
        queues: queue_names,
    };
    HttpResponse::Ok().json(response)
}

#[get("/queues/{queue_name}/message/")]
pub async fn fetch_message(
    redis_pool: web::Data<StaticRedisPool>,
    path: web::Path<QueuePath>,
) -> impl Responder {
    // Get client from pool
    let redis_client = redis_pool.next();

    // Attempt to pull message
    let popped_message_id = match pop_message_from_queue(redis_client, &path.queue_name).await {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::NotFound()
                .body(format!("No messages found in queue: {}", path.queue_name))
        }
    };

    // Add message to processing set
    let _is_message_added_to_processing =
        match add_message_to_processing_set(redis_client, &popped_message_id).await {
            Ok(value) => value,
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .body("Could not start processing message")
            }
        };

    // Get message data
    let message_content = match get_message_content(redis_client, &popped_message_id).await {
        Ok(value) => value,
        Err(e) => return HttpResponse::InternalServerError().body(format!("{:?}", e)),
    };

    let response = MessageContentResponse {
        id: popped_message_id,
        content: message_content,
    };

    HttpResponse::Ok().json(response)
}

#[post("/queues/{queue_name}/message/")]
pub async fn post_message(
    redis_pool: web::Data<StaticRedisPool>,
    path: web::Path<QueuePath>,
    body: web::Json<PostMessageBody>,
) -> impl Responder {
    // Get client from pool
    let redis_client = redis_pool.next();

    // Attempt to create queue, will not affect anything
    // if exists already. Faster than checking then creating.
    let _register_queue_result = match register_queue(redis_client, &path.queue_name).await {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Unable to check for queue existence.")
        }
    };

    // Build message
    let message = Message::new(&path.queue_name, body.into_inner().content);

    // Save message
    let saved_message_id = match save_message(redis_client, message).await {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}", err);
            return HttpResponse::InternalServerError().body("Unable to create message.");
        }
    };

    // Queue message
    let _add_message_response =
        match add_message_to_queue(redis_client, &saved_message_id, &path.queue_name).await {
            Ok(value) => value,
            Err(_) => return HttpResponse::InternalServerError().body("Unable to queue message."),
        };

    let response = PublishMessageResponse {
        id: saved_message_id,
    };

    HttpResponse::Ok().json(response)
}
