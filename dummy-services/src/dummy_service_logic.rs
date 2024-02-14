use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

async fn hello(service_name: Data<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world from {}!", service_name.get_ref()))
}

pub async fn prepare_server(service_name: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(service_name.clone()))
            .route("/", web::get().to(hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
