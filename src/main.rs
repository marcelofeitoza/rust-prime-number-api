#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;

use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;

#[derive(Serialize)]
struct NumberResponse {
    number: u64,
    is_prime_number: bool,
    execution_in_micro_s: u128,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/isPrime?<number>")]
fn get_is_prime(number: u64) -> Json<NumberResponse> {
    let now = SystemTime::now();

    Json(NumberResponse {
        number,
        is_prime_number: is_prime(number),
        execution_in_micro_s: now.elapsed().unwrap().as_micros(),
    })
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }

    return true;
}

#[rocket::main]
async fn main() {
    let mut config = rocket::Config::default();
    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    let _ = rocket::build()
        .configure(config)
        .mount("/", routes![index, get_is_prime])
        .launch()
        .await;
}
