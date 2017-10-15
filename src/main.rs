#[macro_use]
extern crate rouille;

use std::{thread, time};

use rouille::websocket;
use rouille::Response;

fn main() {
    println!("Now listening on 127.0.0.1:8000");
    rouille::start_server("127.0.0.1:8000", move |request| {
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
