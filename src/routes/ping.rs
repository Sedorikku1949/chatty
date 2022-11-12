use crate::structure::{ Request, Response };

pub fn listener(req: &mut Request){
  let mut res = Response::new();

  let data = "Hello World!";
  let _ = res.set_status(200);

  res.add_header("Content-Type".to_string(), "text/plain".to_string());
  res.add_header("Content-Length".to_string(), data.as_bytes().len().to_string());

  res.set_data(data.to_string());

  match res.write_all(&mut req.stream) {
    Ok(_) => {},
    Err(err) => println!("\x1b[0m[Writing Stream Error] {}\x1b[0m", err)
  }
}