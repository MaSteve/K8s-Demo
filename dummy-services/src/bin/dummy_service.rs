use dummy_services_lib::dummy_service_logic::prepare_server;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let service_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "unknown_dummy_service".to_string()
    };
    prepare_server(service_name).await
}
