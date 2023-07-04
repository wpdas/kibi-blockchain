/**
 * Blockchain V3
 * 
 * TODO: melhorar o retorno das rotas com status certo
 * TODO: Separar o modulo do blockchain do modulo de API de rotas
 * TODO: Quando terminar, tentar usar o método de assinatura
 * 
 * TODO:
 * from, to, amount
 */

// Server - API
extern crate rocket;
use rocket::{routes, launch, get};
// use rocket::tokio::time::{sleep, Duration};

mod routes;

// Blockchain - Kibi (yes this is my blockchain name)
mod kibi;

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited for {} seconds", seconds)
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::health_route::get])
        .mount("/new_transaction", routes![routes::new_transaction::post])
        .mount("/chain", routes![routes::get_chain::get])
        .mount("/mine", routes![routes::mine_unconfirmed_transactions::get])
        // .mount("/", routes![delay])
}

// #[rocket::post("/spawn", format = "application/json", data = "<text>")]
// async fn spawn_endpoint(text: String) -> Result<String, ()> {
//     run_on_current_thread(123);

//     Ok(text.to_uppercase()) <<<==================== VEJA ISSO E USE COMO EXEMPLO
// }