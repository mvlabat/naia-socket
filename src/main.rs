
extern crate log;

mod app;
mod loop_native;

use log::info;
use gaia_client_socket::{Config, find_my_ip_address};
use app::App;

const SERVER_PORT: &str = "3179";

fn main() {
    // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
    simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");

    info!("Client Example Started");

    let config = Config::default();

    let server_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

    let mut app = App::new(&server_socket_address, Some(config));

    loop_native::start_loop(&mut app);
}