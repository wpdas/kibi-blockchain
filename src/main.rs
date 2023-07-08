/**
 * Blockchain V3
 */

// Server - API
extern crate rocket;
use rocket::{routes, launch};

mod routes;

// Blockchain - Kibi (yes this is my blockchain name)
mod kibi;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::health_route::get])
        .mount("/new_transaction", routes![routes::new_transaction::post])
        .mount("/contract_transaction", routes![routes::contract_transaction::post])
        .mount("/contract_payload", routes![routes::get_contract_payload::get])
        .mount("/chain", routes![routes::get_chain::get])
        .mount("/mine", routes![routes::mine_unconfirmed_transactions::get])
        .mount("/create_account", routes![routes::new_account::post])
}