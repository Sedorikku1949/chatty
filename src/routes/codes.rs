use crate::structure::{ Request, Response };

pub fn listener_404(req: &mut Request){
  let mut res = Response::new();

  let data = format!("{{ \"message\": \"Cannot find this route\", \"route\": \"{}\" }}", req.url);
  let _ = res.set_status(404);

  res.add_header("Content-Type".to_string(), "application/json".to_string());
  res.add_header("Content-Length".to_string(), data.as_bytes().len().to_string());

  res.set_data(data);

  match res.write_all(&mut req.stream) {
    Ok(_) => {},
    Err(err) => println!("\x1b[0m[Writing Stream Error] {}\x1b[0m", err)
  }
}