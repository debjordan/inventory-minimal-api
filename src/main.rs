use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

/*
actix_web: usado para criar o servidor HTTP e definir rotas.
serde: para serialização/deserialização de estruturas (necessário para manipular JSON).
uuid: gera IDs únicos para os itens do inventário.
 */

#[derive(Serialize, Deserialize)]
struct InventoryItem {
    name: String,
    quantity: u32,
}
// database
// Mutex: usado para gerenciar a concorrencia.
type Inventory = std::sync::Mutex<Vec<InventoryItem>>;

// endpoints
async fn list_items(data: web::Data<Inventory>) -> impl Responder {
    let items = data.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

async fn add_item(item: web::Json<InventoryItem>, data: web::Data<Inventory>) -> impl Responder {
    let mut items = data.lock().unwrap();
    let new_item = InventoryItem {
        name: item.name.clone(),
        quantity: item.quantity,
    };
    items.push(new_item);
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let inventory = web::Data::new(std::sync::Mutex::new(Vec::<InventoryItem>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(inventory.clone())
            .route("/items", web::get().to(list_items))
            .route("/items", web::post().to(add_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
