use pizza_dough_calculator::startup::configure_server;
use std::net::TcpListener;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = configure_server(listener)
        .expect(format!("Failed to configure server on {}", &address).as_str());
    let _ = actix_web::rt::spawn(server);
    address // to be used in route building in tests
}
