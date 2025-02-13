#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

        use wasm_bindgen::prelude::*;

        use naia_socket_client_demo_app::App;

        mod loop_wasm;

        #[wasm_bindgen(start)]
        pub fn main_js() {
            // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
            wasm_logger::init(wasm_logger::Config::default());

            loop_wasm::start_loop(App::new());
        }
    } else {}
}
