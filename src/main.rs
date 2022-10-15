#[macro_use]
extern crate rocket;

mod entities;
mod errors;
mod routes;
mod services;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount(
            "/",
            routes![routes::service::index, routes::service::is_dog],
        )
        .launch()
        .await?;
    Ok(())
}
