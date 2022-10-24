use rocket::fairing::AdHoc;
use crate::{AppError, ManagerPtr};

pub fn load_latest_dataset() -> AdHoc {
    AdHoc::on_liftoff("Load latest dataset", |rocket| {
        Box::pin(async move {
            let _ = rocket
                .state::<ManagerPtr>()
                .map(|m| {
                    println!("Load latest dataset:");
                    println!("   >> Loading latest dataset");
                    if let Err(e) = m.load_latest_dataset() {
                        println!("   >> Loading failed");
                        e.print_stacktrace();
                    } else {
                        println!("   >> Loading successful");
                    }
                });
        })
    })
}
