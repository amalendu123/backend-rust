use std::fmt::format;

use actix_web::{get, patch, post, web::{Data, Json, Path}, App, HttpResponse, HttpServer, Responder};
use db::Database;
use models::{getBlogRequest, update_blog_url, Blog::blog};
use validator::Validate;
mod models;
mod db;
use uuid;
#[get("/getblog")]
async fn get_blog(db:Data<Database>) -> impl Responder {
    let blogs = db.get_all_blogs().await;
    match blogs{
        Some(found_blog)=>HttpResponse::Ok().json(found_blog),
        None=> HttpResponse::Ok().body("Error")
    }
}

#[post("/postblog")]
async fn postblog(body: Json<getBlogRequest>,db:Data<Database>) -> impl Responder{
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let title= body.Blog_title.clone();
            let author = body.author.clone();
            let content = body.content.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_blog = db.add_blog(blog::new(String::from(new_uuid),title,  author, content)).await;
            
            match new_blog{
                Some(created)=> HttpResponse::Ok().body(format!("Created new blog")),
                
                None => HttpResponse::Ok().body("Error creating new blog"),
            }
        },
        Err(_) => HttpResponse::BadRequest().body("Blog content required"),
    }
}

#[patch("/updateblog/{uuid}")]
async fn update_blog(update_blog :Path<update_blog_url>) -> impl Responder{
    let uuid = update_blog.into_inner().uuid;
    HttpResponse::Ok().body(format!("update blog with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let db = Database::init()
                                            .await
                                            .expect("error connection to database");
    let db_data = Data::new(db);
    HttpServer::new(move ||{
        App::new()
            .app_data(db_data.clone())
            .service(get_blog)  
            .service(postblog)
            .service(update_blog)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}