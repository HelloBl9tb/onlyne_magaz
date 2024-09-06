use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::models::User;
use crate::service::UserService;

async fn get_user(id: i32) -> impl Responder {
    let service = UserService::new().await;
    let user = service.get_user(id).await;

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//            .service(web::resource("/users/{id}").route(web::get().to(get_user)))
//     })
//    .bind("127.0.0.1:8080")?
//    .run()
//    .await
// }