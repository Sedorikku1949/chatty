#![feature(unboxed_closures)]
#![feature(const_trait_impl)]

#[macro_use] extern crate magic_crypt;

use std::{
    sync::{Arc, Mutex, self},
    net::TcpListener,
    thread
};
use chrono::{ DateTime, Utc };

use crate::{ structure::Storage, interfaces::user::User, query::new_user };

mod constants;
mod handler;
mod structure;
mod routes;
mod security;
mod interfaces;
pub mod query;


pub fn actual_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%H:%M:%S %d/%m/%Y").to_string()
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{addr}:{port}", addr = constants::LISTENER_ADRESS, port = constants::PORT))?;
    println!("\x1b[2m({date})\x1b[0m \x1b[32mTcpListener ready to work\x1b[0m\n     \x1b[34mListenning on {domain}:{port}\x1b[0m", date = actual_date(), domain = &constants::LISTENER_ADRESS, port = &constants::PORT);

    let storage = Storage::new();
    let arc_storage = Arc::new(Mutex::new(storage));

    let (tx, _rx) = sync::mpsc::channel::<Storage>();

    // test
    //let (test_data, _test_tx) = (Arc::clone(&arc_storage), tx.clone());
    //thread::spawn(move || {
    //    // Lock and get Mutex Storage instance
    //    let mut mutex_storage = test_data.lock().unwrap();
    //    let now = std::time::Instant::now();
    //    //let _ = new_user(&mut mutex_storage, "Sedorriku".to_string(), "cedriccolinc@yahoo.com".to_string(), "ABHI12ki".to_string());
    //    //let res = User::from_query(&mut mutex_storage, &"309e5c1f7cd544078e46c8d24cd488".to_string());
    //    let res = User::from_token(&mut mutex_storage, &"#9w+#zvC/bbn*/s6#=ub59f=F6XAxGL+=vZq=6rWpMZWydgvdcSGbKIQAmdHpBPm#yuez3O+12KNZqN.KlJ9Ua88ONYNx7iPxf&GUL8iR5fxcDC4GtYPxjokU/s2al8Y".to_string());
    //    println!("[{}] {}",  now.elapsed().as_micros(), res.as_ref().unwrap().name);
    //});

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(incm) => {
                // prepare Mutex Storage instance
                let (data, tx) = (Arc::clone(&arc_storage), tx.clone());
                // spawn the thread
                thread::spawn(move || {
                    // Lock and get Mutex Storage instance
                    let mut mutex_storage = data.lock().unwrap();
                    handler::handle_request(incm, &mut mutex_storage, &tx);
                });
            },
            Err(err) => println!("{}", err)
        };
    }

    Ok(())
}