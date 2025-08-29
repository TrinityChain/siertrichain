use siertrichain::core::nft_manager::NFTManager;
use std::sync::{Arc, Mutex};
use tiny_http::{Server, Response};

fn main() {
    println!("Starting the Triangle NFT Marketplace!");

    let nft_manager = Arc::new(Mutex::new(NFTManager::new()));

    let server = Server::http("127.0.0.1:8080").unwrap();

    println!("Marketplace server running on port 8080.");

    for request in server.incoming_requests() {
        let url = request.url();
        match url {
            "/health" => {
                let response = Response::from_string("OK");
                request.respond(response).unwrap();
            }
            _ => {
                let response = Response::from_string("Not Found").with_status_code(404);
                request.respond(response).unwrap();
            }
        }
    }
}
