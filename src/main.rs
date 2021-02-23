extern crate diesel;

use std::thread;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

fn main() {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost");
    let pool = diesel::r2d2::Pool::builder().max_size(30).build(manager).expect("Failed to create pool.");

    for _ in 0..30i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();
            assert!(connection.is_ok());
        });
    }
}