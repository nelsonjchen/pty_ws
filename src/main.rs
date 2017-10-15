#[macro_use]
extern crate rouille;
extern crate nix;
extern crate nix_ptsname_r_shim;
extern crate multiqueue;

use std::{thread, time};

use rouille::websocket;
use rouille::Response;

fn main() {
    let (send, recv) = multiqueue::mpmc_queue(10);

    println!("Now listening on 127.0.0.1:8000");
    rouille::start_server("127.0.0.1:8000", move |request| {
        recv;
        router!(request,
            (GET) (/) => {
                Response::empty_404()
            },

            (GET) (/ws) => {
                let (response, websocket) = try_or_400!(websocket::start(&request, None::<&str>));
                thread::spawn(move || {
                    let ws = websocket.recv().unwrap();
                    websocket_handling_thread(ws);
                });
                response
            },
            _ => rouille::Response::empty_404()
        )
    });
}

fn websocket_handling_thread(mut websocket: websocket::Websocket) {
    loop {
        thread::sleep(time::Duration::from_secs(1));
        let res = websocket.send_text("Hello");
        match res {
            Ok(()) => {}
            Err(_) => {
                println!("oh no!");
                break
            }
        }
    }
}
