extern crate bus;
extern crate nix;
extern crate nix_ptsname_r_shim;
#[macro_use]
extern crate rouille;
use bus::{Bus, BusReader};

use std::thread;
use std::sync::{Arc, Mutex};

use rouille::websocket;
use rouille::Response;

fn main() {
    let bus_mutex = Arc::new(Mutex::new(Bus::new(5)));

    let loop_bus_mutex = bus_mutex.clone();
    thread::spawn(move || {
        pty_handling_thread(loop_bus_mutex);
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

fn pty_handling_thread(bus: Arc<Mutex<Bus<Vec<u8>>>>) {
    use nix::fcntl::O_RDWR;
    use nix::pty::{grantpt, unlockpt};
    use nix_ptsname_r_shim::ptsname_r;
    let master_fd = nix::pty::posix_openpt(O_RDWR).unwrap();
    grantpt(&master_fd).unwrap();
    unlockpt(&master_fd).unwrap();
    let slave_name = ptsname_r(&master_fd).unwrap();
    println!("Slave name: {}", slave_name);
    // Open the slave name so it never "ends" like in openpty.
    let _file = File::create(&slave_name).unwrap();

    use std::fs::File;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    use std::io::prelude::*;

    let mut file = unsafe { File::from_raw_fd(master_fd.as_raw_fd()) };
    let mut buffer = [0; 1024];
    loop {
        let read = file.read(&mut buffer).unwrap();
        let mut broadcast_slice = Vec::new();
        broadcast_slice.extend_from_slice(&buffer[0..read]);
        bus.lock().unwrap().broadcast(broadcast_slice);
    }
}

fn websocket_handling_thread(mut websocket: websocket::Websocket, mut recv: BusReader<Vec<u8>>) {
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
