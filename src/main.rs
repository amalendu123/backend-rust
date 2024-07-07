use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use models::getBlogRequest;
use validator::Validate;
mod models;

#[get("/getblog")]
async fn get_blog(body: Json<getBlogRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let blog = body.content.clone();
            HttpResponse::Ok().body(format!("Blog entered is {}", blog))
        },
        Err(_) => HttpResponse::BadRequest().body("Blog content required"),
    }
}

#[post("/postblog")]
async fn postblog() -> impl Responder{
    HttpResponse::Ok().body("Post a Blogs")
}

#[patch("/updateblog/{uuid}")]
async fn update_blog() -> impl Responder{
    HttpResponse::Ok().body("update blog")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(get_blog)  
            .service(postblog)
            .service(update_blog)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}