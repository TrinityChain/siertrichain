use siertrichain::core::nft_manager::NFTManager;
use std::sync::{Arc, Mutex};
use tiny_http::{Server, Response, Method, Request, Header};
use serde_json::json;

fn main() {
    println!("Starting the Triangle NFT Marketplace!");

    let nft_manager = Arc::new(Mutex::new(NFTManager::new()));
    let server = Server::http("127.0.0.1:8080").unwrap();

    println!("Marketplace server running on port 8080.");

    for request in server.incoming_requests() {
        handle_request(request, Arc::clone(&nft_manager));
    }
}

fn handle_request(request: Request, nft_manager: Arc<Mutex<NFTManager>>) {
    match (request.method(), request.url()) {
        (&Method::Get, "/health") => {
            let response = Response::from_string("OK");
            request.respond(response).unwrap();
        }
        (&Method::Get, "/nfts") => {
            // List NFTs (stub)
            let nfts = {
                let manager = nft_manager.lock().unwrap();
                manager.list_nfts() // You need to implement this in NFTManager
            };
            let body = json!({ "nfts": nfts }).to_string();
            let response = Response::from_string(body)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            request.respond(response).unwrap();
        }
        (&Method::Post, "/mint") => {
            // Mint NFT (stub)
            let result = {
                let mut manager = nft_manager.lock().unwrap();
                manager.mint_nft() // You need to implement this in NFTManager
            };
            let body = json!({ "result": result }).to_string();
            let response = Response::from_string(body)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            request.respond(response).unwrap();
        }
        (&Method::Post, "/buy") => {
            // Buy NFT (stub)
            let result = {
                let mut manager = nft_manager.lock().unwrap();
                manager.buy_nft() // You need to implement this in NFTManager
            };
            let body = json!({ "result": result }).to_string();
            let response = Response::from_string(body)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            request.respond(response).unwrap();
        }
        _ => {
            let response = Response::from_string("Not Found").with_status_code(404);
            request.respond(response).unwrap();
        }
    }
}