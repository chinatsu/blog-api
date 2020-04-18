#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
use hyper::{
    service::make_service_fn, service::service_fn, Server
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;

mod routes;
mod db;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 8080).into();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let ny_service = make_service_fn(move |_| {
        let pool = pool.clone();
        async {
             Ok::<_, GenericError>(service_fn(move |req| {
                routes::route(req, pool.get().unwrap())
             }))
        }
     });

     let server = Server::bind(&addr).serve(ny_service);

     info!("Listening on http://{}", addr);

     server.await?;

     Ok(())
}
