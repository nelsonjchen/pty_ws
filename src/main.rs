extern crate bus;
extern crate nix;
extern crate nix_ptsname_r_shim;
#[macro_use]
extern crate rouille;
use bus::{Bus, BusReader};

use std::{thread, time};
use std::sync::{Arc, Mutex};

use rouille::websocket;
use rouille::Response;

fn main() {
    let bus_mutex = Arc::new(Mutex::new(Bus::new(5)));

    let loop_bus_mutex = bus_mutex.clone();
    thread::spawn(move || {
        let mut counter = 0;
        loop {
            thread::sleep(time::Duration::from_millis(100));
            let mut bus = loop_bus_mutex.lock().unwrap();
            println!("Broadcasting {}", counter);
            bus.broadcast(counter);
            counter += 1;
        }
    });

    let server_bus_mutex = bus_mutex.clone();
    println!("Now listening on 127.0.0.1:8000");
    rouille::start_server("127.0.0.1:8000", move |request| {
        router!(request,
            (GET) (/) => {
                Response::empty_404()
            },

            (GET) (/ws) => {
                let (response, websocket) = try_or_400!(websocket::start(&request, None::<&str>));
                let request_mutex = server_bus_mutex.clone();
                let bus_reader = request_mutex.lock().unwrap().add_rx();
                thread::spawn(move || {
                    let ws = websocket.recv().unwrap();
                    websocket_handling_thread(ws, bus_reader);
                });
                response
            },
            _ => rouille::Response::empty_404()
        )
    });
}

fn websocket_handling_thread(mut websocket: websocket::Websocket, mut recv: BusReader<u8>) {
    loop {
        let data = match recv.recv() {
            Ok(d) => d,
            _ => break,
        };
        match websocket.send_text(&format!("{:?}", data)) {
            Ok(_) => {}
            _ => {
                println!("Got dropped");
                break;
            }
        };
    }
}
