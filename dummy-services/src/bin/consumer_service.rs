use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use serde_json::json;

async fn request_handler() -> impl Responder {
    let req1 = reqwest::get("http://service1.devo/");
    let req2 = reqwest::get("http://service2.devo/");
    let req3 = reqwest::get("http://service3.devo/");

    match (req1.await, req2.await, req3.await) {
        (Ok(res1), Ok(res2), Ok(res3)) => {
            let response = json!({
                "res1": res1.text().await.unwrap_or_default(),
                "res2": res2.text().await.unwrap_or_default(),
                "res3": res3.text().await.unwrap_or_default()
            });
            HttpResponse::Ok().json(response)
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

async fn prepare_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(request_handler)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    prepare_server().await
}
