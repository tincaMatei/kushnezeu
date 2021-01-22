#[macro_use]
extern crate diesel;
#[macro_use]
extern crate backend;
#[macro_use]
extern crate diesel_migrations;

use self::backend::*;
use self::models::*;
use self::diesel::prelude::*;
use self::database::DatabaseServer;

use tide::Request;
use tide::prelude::*;

pub mod database;

// Return a hex token of n bytes
fn lazy_token(n: usize) -> String {
    const CHARSET: &[u8] = b"abcdef0123456789";
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let token: String = (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    token
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let server = DatabaseServer::start_database();
    
    server.run_migrations();
    
    let mut app = tide::with_state(server);
    
    app.at("*").get(|req: tide::Request<DatabaseServer>| async move {
        println!("{}", req.url().path().to_string());
        Ok("Ery nice!")
    });

    app.at("*").post(|req| async move {
        Ok("Milsugi B^)")
    });

    app.at("/login").post(|mut req: tide::Request<DatabaseServer>| async move {
        let mut user: backend::models::User = req.body_form().await?;
        
        user.username = user.username.to_lowercase();

        use schema::users::dsl::*;
        let results = users.filter(username.eq(user.username))
            .limit(1)
            .load::<User>(&req.state().connect().unwrap())
            .expect("Error loading post");
        
        println!("{:?}", results);
        
        // We found an entry in the database of the given user
        if(results.len() == 1) {
            if(results[0].password == user.password) { // The password matches
                // We create a session_id token
                let token = lazy_token(64);

                // We should insert this token into the database

                Ok(json!( {
                    "error" : false,
                    "error_msg" : "Login successful",
                    "session_id" : token
                } ))
            } else {
                Ok(json!( {
                    "error" : true,
                    "error_msg" : "Wrong username or password",
                    "session_id" : ""
                } ))
            }
        } else {
            Ok(json!( {
                "error" : true,
                "error_msg" : "Wrong username or password",
                "session_id" : ""
            } ))
        }
    });

    app.at("/new-account").post(|mut req: tide::Request<DatabaseServer>| async move {
        let user: backend::models::User = req.body_form().await?;
        let new_user = NewUser {
            username: &user.username.to_lowercase(),
            password: &user.password
        };

        use schema::users;
        let result = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<(i32, String, String)>(&req.state().connect().unwrap())
            .expect("Error creating new user");
        Ok(format!("Created new user! {:?}", result))
    });
    
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

