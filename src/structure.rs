use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream, process::exit,
};

use crate::{constants::{ MYSQL_USER, MYSQL_PASSWORD, MYSQL_DOMAIN, MYSQL_DB, self }, security, actual_date};
use mysql::{Pool, PooledConn};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Method {
    Post,
    Get,
    Delete,
    Put,
    Head,
    Connect,
    Options,
    Trace,
    Patch,
    Unknown,
}

impl Method {
    #[allow(dead_code)]
    pub fn as_str(&self) -> String {
        match self {
            Method::Post => "POST".to_string(),
            Method::Get => "GET".to_string(),
            Method::Delete => "DELETE".to_string(),
            Method::Put => "PUT".to_string(),
            Method::Head => "HEAD".to_string(),
            Method::Connect => "CONNECT".to_string(),
            Method::Options => "OPTIONS".to_string(),
            Method::Trace => "TRACE".to_string(),
            Method::Patch => "PATCH".to_string(),
            Method::Unknown => "Unknown".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub stream: TcpStream,
    pub socket: String,
    pub http_version: String,

    pub method: Method,
    pub url: String,
    pub body: String,
    pub header: HashMap<String, String>,
}

impl Request {
    pub fn new(mut stream: &TcpStream) -> Request {
        let client = stream
            .peer_addr()
            .unwrap()
            .to_string()
            .splitn(2, ":")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let mut req = Request {
            stream: stream.try_clone().unwrap(),
            socket: (&*client.get(0).unwrap()).to_string(),
            http_version: String::new(),

            method: Method::Unknown,
            url: String::new(),
            body: String::new(),
            header: HashMap::new(),
        };
        let buf = BufReader::new(&mut stream);
        let header = buf
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();
        for h in &header {
            if h != header.get(0).unwrap() {
                let parsed = h.splitn(2, ": ").map(|e| e.trim()).collect::<Vec<&str>>();
                req.header
                    .insert(parsed[0].to_string(), parsed[1].to_string());
            } else {
                // first line, contain http version, method and url
                let fh = &h
                    .splitn(3, ' ')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();
                for i in 0..fh.len() {
                    let default_e = &String::new();
                    let e: &String = fh.get(i).unwrap_or(default_e);
                    match i {
            0 => {
              // method
              req.method = match e.as_str() {
                "GET" => Method::Get,
                "POST" => Method::Post,
                "HEAD" => Method::Head,
                "PUT" => Method::Put,
                "DELETE" => Method::Delete,
                "CONNECT" => Method::Connect,
                "OPTIONS" => Method::Options,
                "TRACE" => Method::Trace,
                "PATCH" => Method::Patch,
                _ => Method::Unknown
              }
            },
            1 => req.url = (&*e).to_owned(),
            2 => req.http_version = (&*e).to_owned(),
            _wtf => println!("\x1b[31m[Unknown data] Unknown information in the top header of the request: {}\x1b[0m", e)
          }
                }
            };
        }

        //if req.method.as_str() == Method::Post.as_str() { stream.read_to_string(&mut req.body); };

        req
    }
}

pub struct Response {
    http_version: HttpVersion,
    status: i32,
    header: HashMap<String, String>,
    data: String,
}

enum HttpVersion {
    Http1_1,
    Http2,
    Http3,
}

impl HttpVersion {
    fn as_str(&self) -> &str {
        match self {
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        }
    }
}

#[allow(dead_code)]
impl Response {
  pub fn new() -> Response {
    Response {
      http_version: HttpVersion::Http1_1,
      status: -1,
      header: HashMap::new(),
      data: String::new(),
    }
  }

  fn valid_status(status: i32) -> bool {
    status > 99 && status < 600 // between 100 and 599: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
  }

  fn check_validity(&self) -> bool {
    if !Response::valid_status(self.status)
      || (self.header.contains_key("Content-Length")
        && !self.header.contains_key("Content-Type"))
      || (self.header.contains_key("Content-Type")
        && !self.header.contains_key("Content-Length"))
    {
      false
    } else {
      true
    }
  }

  fn is_numeric(&self, str: &String) -> bool {
    for c in str.chars() {
      if !c.is_numeric() { return false; }
    }
    true
  }

  fn format_header(&self, k: &String, v: &String) -> String {
    if self.is_numeric(v) { format!("{key}: {value}", key = k, value = v.replace("'", "\\'")) }
    else  { format!("{key}: '{value}'", key = k, value = v.replace("'", "\\'")) }
  }

  pub fn format_res(&self) -> String {
    let formatted_header = self.header.iter().map(|(k, v)| self.format_header(k, v)).collect::<Vec<String>>().to_vec().join("\r\n");
    format!(
      "{http_version} {status}\r\n{header}\r\n{data}",
      http_version = self.http_version.as_str(),
      status = self.status,
      header = if self.header.len() < 1 { "\r\n".to_string() } else { formatted_header + "\r\n" },
      data = self.data
    )
  }

  pub fn set_http(&mut self, http: String) -> Result<(), String> {
    if http == HttpVersion::Http1_1.as_str()    { self.http_version = HttpVersion::Http1_1; Ok(()) }
    else if http == HttpVersion::Http2.as_str() { self.http_version = HttpVersion::Http2;   Ok(()) }
    else if http == HttpVersion::Http3.as_str() { self.http_version = HttpVersion::Http3;   Ok(()) }
    else {  Err("Unknown HTTP version was provided".to_string()) }
  }

  pub fn set_status(&mut self, status: i32) -> Result<(), String> {
    if !Response::valid_status(status) { Err("Invalid status code provided, please refer to the standart RFC 9110".to_string()) }
    else { self.status = status; Ok(()) }
  }

  pub fn add_header(&mut self, name: String, data: String){
    if self.header.contains_key(&name) { self.header.remove(&name); };
    self.header.insert(name, data);
  }

  pub fn remove_header(&mut self, name: &String){
    self.header.remove(name);
  }

  pub fn set_data(&mut self, data: String){
    self.data = data
  }
  pub fn push_data(&mut self, data: String){
    self.data.push_str(data.as_str())
  }

  pub fn write_all(&self, stream: &mut TcpStream) -> Result<(), String> {
    if !self.check_validity() {
      Err("Cannot write response with invalid data, please ensure that the response is correctly build".to_string())
    } else {
      match stream.write_all(&self.format_res().as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
      }
    }
  }
}



fn load_mysql() -> Result<PooledConn, String> {
  match security::decrypt(constants::SECURITY_KEY, MYSQL_PASSWORD) {
    Ok(password) => {
      let url = format!("mysql://{u}:{p}@{d}/{n}", u = MYSQL_USER, p = password, d = MYSQL_DOMAIN, n = MYSQL_DB);
      match Pool::new(url.as_str()) {
          Ok(pool) => {
              match pool.get_conn() {
                  Ok(conn) => {
                    println!("\x1b[2m({date})\x1b[0m \x1b[32mMySQL connection ready to work\x1b[0m", date = actual_date());
                    Ok(conn)
                  },
                  Err(_) => Err("Cannot conntct to database".to_string()) 
              }
          },
          Err(_) => Err("Cannot create MySQL pool".to_string()),
      }
    },
    Err(_) => {
      println!("[\x1b[35mMySQL\x1b[0m] \x1b[31mCannot resolve the decrypted password, security breach found.\x1b[0m");
      exit(0)
    }
  }
  
}

pub struct Storage {
  maintenance: bool,
  pub database: PooledConn
}

#[allow(dead_code)]
impl Storage {
  pub fn new() -> Storage {
    match load_mysql() {
      Ok(conn) => {
        Storage {
          maintenance: false,
          database: conn
        }
      },
      Err(_) => panic!("Cannot connect to MySQL instance")
    }
  }

  pub fn maintenance(&self) -> &bool {
    &self.maintenance
  }
  pub fn maintenance_mut(&mut self) -> &mut bool {
    &mut self.maintenance
  }

  pub fn set_maintenance(&mut self, new_value: bool) {
    self.maintenance = new_value;
  }
}