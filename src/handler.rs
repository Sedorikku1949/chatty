use std::{net::TcpStream, sync::{MutexGuard, mpsc::Sender}};

use crate::{structure::{Request, Storage, Response}, routes::{ping, codes::listener_404}, actual_date};

pub fn handle_request(stream: TcpStream, storage: &mut MutexGuard<Storage>, _channel: &Sender<Storage>){
  // create Request and signal that stream was received
  let mut req = Request::new(&stream);
  println!("\x1b[2m({date})\x1b[0m Input stream received", date = actual_date());

  // process
  if *storage.maintenance() {
    // maintenance !
    let mut res = Response::new();
    let data = format!("{{ \"message\": \"The service is currently unavailable, retry later\" }}");
    let _ = res.set_status(503); // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/503
    res.add_header("Content-Type".to_string(), "application/json".to_string());
    res.add_header("Content-Length".to_string(), data.as_bytes().len().to_string());
    res.set_data(data);
    match res.write_all(&mut req.stream) {
      Ok(_) => {},
      Err(err) => println!("\x1b[0m[Writing Stream Error] {}\x1b[0m", err)
    }
  } else {
    // let's process
    let path: Vec<&str> = req.url.splitn(2, "?").map(|w| w.trim()).collect::<Vec<&str>>();
    match path.get(0) {
      Some(&"/ping") => { ping::listener(&mut req) },
      _ => { listener_404(&mut req) }
    }
  }
}