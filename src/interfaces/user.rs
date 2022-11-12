#![allow(dead_code)]
use std::str::FromStr;

use chrono::{ Utc, DateTime, NaiveDateTime };
use mysql::prelude::Queryable;
use num::{bigint::Sign, BigInt, FromPrimitive};

use crate::structure::Storage;

pub enum AcceptFriendRequest {
  No,
  OnlyFromFriendOfFriends,
  Yes,
  Unknown
}

impl AcceptFriendRequest {
  pub fn from(i: i32) -> AcceptFriendRequest {
    match i {
      0 => AcceptFriendRequest::No,
      1 => AcceptFriendRequest::OnlyFromFriendOfFriends,
      2 => AcceptFriendRequest::Yes,
      _ => AcceptFriendRequest::Unknown
    }
  }
}

type UserFetchQuery = (String, String, String, String, String, String, i32, i32, bool, bool, i32, i32);

pub struct User {
  pub id: String,
  email: String,
  pub name: String,
  password: String,
  pub token: String,
  pub permissions: num::BigInt,
  pub system: bool,
  pub disabled: bool,
  pub creation: DateTime<Utc>,
  pub accept_friend_request: AcceptFriendRequest,
  pub xp: i32,
  pub lvl: i32,

  pub valid: bool
}

impl User {
  pub fn new() -> User {
    User {
      id: String::new(),
      email: String::new(),
      name: String::new(),
      password: String::new(),
      token: String::new(),
      permissions: num::BigInt::new(Sign::Plus, Vec::new()),
      system: false,
      disabled: true,
      creation: Utc::now(),
      accept_friend_request: AcceptFriendRequest::No,
      xp: 0,
      lvl: 0,

      valid: false
    }
  }

  /**
   * +------------------------+---------------------+------+-----+---------------------+-------+
   * | Field                  | Type                | Null | Key | Default             | Extra |
   * +------------------------+---------------------+------+-----+---------------------+-------+
   * | id                     | char(30)            | NO   | PRI | NULL                |       |
   * | email                  | varchar(350)        | NO   |     | NULL                |       |
   * | password               | char(64)            | NO   |     | NULL                |       |
   * | token                  | char(128)           | NO   |     | NULL                |       |
   * | creation               | datetime            | NO   |     | current_timestamp() |       |
   * | name                   | varchar(32)         | NO   |     | NULL                |       |
   * | permissions            | bigint(20) unsigned | NO   |     | 0                   |       |
   * | accept_friends_request | int(11)             | NO   |     | 2                   |       |
   * | system                 | tinyint(1)          | NO   |     | 0                   |       |
   * | disabled               | tinyint(1)          | NO   |     | 0                   |       |
   * | xp                     | int(11)             | NO   |     | 0                   |       |
   * | lvl                    | int(11)             | NO   |     | 0                   |       |
   * +------------------------+---------------------+------+-----+---------------------+-------+
   * 12 rows
   *
   */
  pub fn from_query(storage: &mut Storage, id: &String) -> Result<User, String> {
    if id.len() != 30 { Err("The ID can only be a 30 long string".to_string()) }
    else {
      match storage.database.query_first::<UserFetchQuery, String>(format!("SELECT * FROM `users` WHERE `id`='{}' LIMIT 1;", id)) {
          Ok(Some(raw)) => {
            // User fetched
            Ok(
              User {
                  id: raw.0,
                  email: raw.1,
                  name: raw.5,
                  password: raw.2,
                  token: raw.3,
                  permissions: BigInt::from_i32(raw.6).unwrap_or(BigInt::default()),
                  system: raw.8,
                  disabled: raw.9,
                  creation: DateTime::from_utc(NaiveDateTime::parse_from_str(&raw.4, "%Y-%m-%d %H:%M:%S").unwrap_or(NaiveDateTime::default()), Utc),
                  accept_friend_request: AcceptFriendRequest::from(raw.7),
                  xp: raw.10,
                  lvl: raw.11,
                  valid: true
              }
            )
          }
          Ok(_) => Err("No user was found with this ID".to_string()),
          _ => Err("A SQL error has occured".to_string())
      }
    }
  }

  pub fn from_token(storage: &mut Storage, token: &String) -> Result<User, String> {
    if token.len() != 128 { Err("The token can only be a 128 long string".to_string()) }
    else {
      match storage.database.query_first::<UserFetchQuery, String>(format!("SELECT * FROM `users` WHERE `token`='{}' LIMIT 1;", token)) {
          Ok(Some(raw)) => {
            // User fetched
            Ok(
              User {
                  id: raw.0,
                  email: raw.1,
                  name: raw.5,
                  password: raw.2,
                  token: raw.3,
                  permissions: BigInt::from_i32(raw.6).unwrap_or(BigInt::default()),
                  system: raw.8,
                  disabled: raw.9,
                  creation: DateTime::from_utc(NaiveDateTime::parse_from_str(&raw.4, "%Y-%m-%d %H:%M:%S").unwrap_or(NaiveDateTime::default()), Utc),
                  accept_friend_request: AcceptFriendRequest::from(raw.7),
                  xp: raw.10,
                  lvl: raw.11,
                  valid: true
              }
            )
          }
          Ok(_) => Err("No user was found with this ID".to_string()),
          _ => Err("A SQL error has occured".to_string())
      }
    }
  }
  
  pub fn set_name(mut self, name: String) -> User {
    self.name = name;
    self
  }
  pub fn set_id(mut self, id: String) -> User {
    self.id = id;
    self
  }
  pub fn set_email(mut self, email: String) -> User {
    self.email = email;
    self
  }
  pub fn set_password(mut self, password: String) -> User {
    self.password = password;
    self
  }
  pub fn set_token(mut self, token: String) -> User {
    self.token = token;
    self
  }
  pub fn set_permissions(mut self, bigint: BigInt) -> User {
    self.permissions = bigint;
    self
  }
  pub fn set_system(mut self, system: bool) -> User {
    self.system = system;
    self
  }
  pub fn set_disabled(mut self, disabled: bool) -> User {
    self.disabled = disabled;
    self
  }
  pub fn set_creation(mut self, t: DateTime<Utc>) -> User {
    self.creation = t;
    self
  }
  pub fn set_friend_request(mut self, status: AcceptFriendRequest) -> User {
    self.accept_friend_request = status;
    self
  }
  pub fn set_xp(mut self, xp: i32) -> User {
    self.xp = xp;
    self
  }
  pub fn set_lvl(mut self, lvl: i32) -> User {
    self.lvl = lvl;
    self
  }
  pub fn valid_user(self) -> Result<User, String> {
    if self.check_validity() { Ok(self) }
    else { Err("Invalid fields in User".to_string()) }
  }
  fn check_validity(&self) -> bool {
    self.name.len() > 0 || self.token.len() > 0 || self.password.len() == 32 || self.email.len() > 0 || self.id.len() == 30
  }

  pub fn get_token(&self) -> String {
    (*self.token).to_string()
  }
}