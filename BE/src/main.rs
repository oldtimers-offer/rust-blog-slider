use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct BlogPost {
    id: i32,
    title: String,
    content: String,
    images: Vec<String>,
}

async fn get_posts() -> impl Responder {
    let posts = vec![
        BlogPost {
            id: 1,
            title: "My First Blog Post".to_string(),
            content: "This is an example blog post with multiple images.".to_string(),
            images: vec!["https://storage.cloud.google.com/rust_blog/slider/1.jpeg".to_string(), "https://storage.cloud.google.com/rust_blog/slider/2.jpeg".to_string()],
        },
        BlogPost {
            id: 2,
            title: "Another Blog Post".to_string(),
            content: "Here is another post with images.".to_string(),
            images: vec!["https://storage.cloud.google.com/rust_blog/slider/3.jpeg".to_string(), "https://storage.cloud.google.com/rust_blog/slider/4.jpeg".to_string()],
        },
    ];
    HttpResponse::Ok().json(posts)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
           .wrap(Cors::permissive())
            .route("/posts", web::get().to(get_posts))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
