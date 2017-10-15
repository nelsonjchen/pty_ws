#[macro_use]
extern crate rouille;
extern crate nix;
extern crate nix_ptsname_r_shim;
extern crate multiqueue;

use std::{thread, time};
use std::sync::Mutex;

use rouille::websocket;
use rouille::Response;

fn main() {
    let (send, recv) = multiqueue::broadcast_queue::<u8>(2);
    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_secs(1));
            match send.try_send(8u8) {
                _ => {}
            }
        }
    });
    let recv_mutex = Mutex::new(recv);

    println!("Now listening on 127.0.0.1:8000");
    rouille::start_server("127.0.0.1:8000", move |request| {
        let recv = recv_mutex.lock().unwrap().clone();
        for _ in recv.try_iter() {}
        let recv_inner = recv.add_stream();
        // Drain

        router!(request,
            (GET) (/) => {
                Response::empty_404()
            },

            (GET) (/ws) => {
                let (response, websocket) = try_or_400!(websocket::start(&request, None::<&str>));
                thread::spawn(move || {
                    let ws = websocket.recv().unwrap();
                    websocket_handling_thread(ws, recv_inner);
                });
                response
            },
            _ => rouille::Response::empty_404()
        )
    });
}

fn websocket_handling_thread(mut websocket: websocket::Websocket, recv: multiqueue::BroadcastReceiver<u8>) {
    loop {
        let data = recv.recv().unwrap();
        let res = websocket.send_text(&format!("{:?}", data));
        match res {
            Ok(()) => {}
            Err(_) => {
                println!("oh no!");
                recv.unsubscribe();
                break
            }
        }
    }

}
