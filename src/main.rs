#[allow(unused)]
mod route;
#[allow(unused)]
mod vo;
#[allow(unused)]
mod exception_handler;
#[allow(unused)]
mod service;

use log::info;
#[allow(unused)]
use crate::route::route;

fn main() {
    start_server();
}
fn start_server() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            init().await;
            info!("Server::run; mode=release");
            warp::serve(route())
                .run(([0, 0, 0, 0], 19876))
                .await;
        })
}
async fn init() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}