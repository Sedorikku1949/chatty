use mysql::prelude::Queryable;

use crate::{
  security::{ generate_random_id, generate_random_str, hash },
  structure::Storage,
};

fn create_user_creation_query(
  name: String,
  email: String,
  password: String,
  id: String,
  token: String,
) -> String {
  format!(
    "INSERT INTO users (id, email, password, token, creation, name, permissions, accept_friends_request, system, disabled, xp, lvl) VALUES ({id:?}, {email:?}, {password:?}, {token:?}, {creation}, {name:?}, {permissions:?}, {accept_friends_request:?}, {system:?}, {disabled:?}, {xp:?}, {lvl:?});",
    id = id,
    email = email,
    password = password,
    token = token,
    creation = "current_timestamp()",
    name = name,
    permissions = "0",
    accept_friends_request = 0,
    system = false,
    disabled = false,
    xp = 0,
    lvl = 0
  )
}

pub fn new_user(
  storage: &mut Storage,
  name: String,
  email: String,
  password: String,
) -> Result<(), String> {
  if name.len() > 32 { Err("Name cannot contain more than 32 caracters".to_string()) }
  else if email.len() > 350 { Err("Invalid email was provided".to_string()) }
  else {
    // let's go
    let encrypted_password = hash((&*password).to_owned());
    drop(password);
    let mut id: String = generate_random_id(30);
    let mut token: String = generate_random_str(128);
    while token.len() != 128 { token = generate_random_id(128) };

    let mut created: bool = false;

    'create_account: loop {
      let qtest_res = storage.database.query_first::<(String, String, String), String>(format!("SELECT `token`, `id`, `name` FROM `users` WHERE token={token:?} OR id={id:?} OR name={name:?}"));
      if let Ok(Some(qtest)) = qtest_res {
        // AYO
        if &qtest.0 == &token { token = generate_random_str(128) };
        if &qtest.1 == &id { id = generate_random_id(30) };
        if &qtest.2 == &name { break 'create_account; };
      } else if let Ok(_) = qtest_res {
        // ok go go
        println!("[{}]", token);
        println!("token_length = {}", token.len());
        let q = create_user_creation_query(name, email, encrypted_password, id, token);
        if let Ok(_) = storage.database.query::<String, String>(q) {
          created = true;
          break 'create_account;
        } else { break 'create_account; }
      } else {
        break 'create_account;
      }
    }
    if created { Ok(()) }
    else { Err("An error as occured while creating user instance".to_string()) }
  }
}