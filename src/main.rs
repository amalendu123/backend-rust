use actix_web::{get, patch, post, web::post, App, HttpResponse, HttpServer, Responder};

#[get("/getblog")]
async  fn get_blog() -> impl Responder{
    HttpResponse::Ok().body("Blogs are available here")
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