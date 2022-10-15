#[macro_use]
extern crate rocket;

mod errors;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let str = "Hello, World!";
    println!("{}", str);
    let _ = rocket::build()
        .mount("/", routes![routes::service::index])
        .launch()
        .await?;
    Ok(())
}
