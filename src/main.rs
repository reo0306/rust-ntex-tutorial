use ntex::web;
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[web::get("/")]
async fn index(data: web::types::State<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .state(AppState {
                app_name: String::from("Ntex"),
            })
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
