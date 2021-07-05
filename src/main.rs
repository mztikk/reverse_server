use rocket::http::Method;
use rocket::{get, post, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

extern crate rocket;

#[post("/reverse", data = "<s>")]
fn reverse(s: &str) -> String {
    s.graphemes(true).rev().collect()
}

#[get("/reversedev/<s>")]
fn reverse_dev(s: &str) -> String {
    s.graphemes(true).rev().collect()
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "https://www.mztikk.de",
        "https://mztikk.de",
        "http://localhost:1234",
    ]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .mount("/", routes![reverse, reverse_dev])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
