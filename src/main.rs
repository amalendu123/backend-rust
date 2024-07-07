use actix_web::{get, patch, post, HttpResponse, HttpServer, Responder,App};

#[get("/getblog")]
async  fn get_blog() -> impl Responder{
    HttpResponse::Ok().body("Blogs are available here")
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(get_blog)  
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}