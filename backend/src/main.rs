extern crate diesel;
extern crate backend;
#[macro_use]
extern crate diesel_migrations;

use self::backend::*;
use self::models::*;
use self::diesel::prelude::*;
use self::database::DatabaseServer;

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

#[derive(Default, Deserialize)]
#[serde(default)]
struct SessionId {
    session_id: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let server = DatabaseServer::start_database();
    
    server.run_migrations();
    
    let mut app = tide::with_state(server);
   
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
        if results.len() == 1 {
            if results[0].password == user.password { // The password matches
                // We create a session_id token
                let token = lazy_token(64);

                // We should insert this token into the database
                
                let mut expire_date = chrono::offset::Utc::now().naive_utc();
                expire_date = expire_date + chrono::Duration::hours(12);
                let new_session = Session {
                    session_id: &token,
                    user_id: results[0].id,
                    expire: expire_date,
                };

                use schema::sessions;
                let result = diesel::insert_into(sessions::table)
                    .values(new_session)
                    .get_result::<(String, i32, chrono::NaiveDateTime)>(&req.state().connect().unwrap());
                
                println!("Added session: {:?}", result);

                if let Err(_) = result {
                    Ok(json!( {
                        "error" : true,
                        "error_msg" : "Database error: Failed to insert session into database",
                        "session_id" : "",
                    } ))
                } else {
                    println!("Expire date: {}", expire_date.to_string());
                    Ok(json!( {
                        "error" : false,
                        "error_msg" : "Login successful",
                        "session_id" : token,
                        "expire" : expire_date.to_string()
                    } ))
                }
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

    app.at("/logout").post(|mut req: tide::Request<DatabaseServer>| async move {
        let session: SessionId = req.body_form().await?;
        println!("Session_id: {}", session.session_id);
        use backend::schema::sessions::dsl::*;
        let expire_date = chrono::offset::Utc::now().naive_utc();
        let num_deleted = diesel::delete(sessions.filter(session_id.eq(&session.session_id).or(&expire.le(expire_date))))
            .execute(&req.state().connect().unwrap())
            .expect("Failed to delete the session");
        println!("Deleted session {}", num_deleted);
        Ok("Erased session")
    });

    app.at("/refresh-session").post(|mut req: tide::Request<DatabaseServer>| async move {
        let session: SessionId = req.body_form().await?;
        
        let expire_date = chrono::offset::Utc::now().naive_utc();
        let expire_date = expire_date + chrono::Duration::hours(12);
        use backend::schema::sessions::dsl::*;
       
        let updated = diesel::update(sessions.find(session.session_id))
            .set(expire.eq(expire_date));
        
        println!("Refreshed session: {:?}", updated);

        Ok(json!( { "expire" : expire_date.to_string() } ))
    });

    // Return the content from the page
    app.at("/content/:group/:page/read").post(|mut req: tide::Request<DatabaseServer>| async move {
        let user_session: SessionId = req.body_form().await?;
        
        let group_name = req.param("group").expect("Failed to get group name");
        let page_name = req.param("page").expect("Failed to get page name");

        use backend::schema::sessions::dsl::*;
        let session = sessions.filter(session_id.eq(user_session.session_id))
            .limit(1)
            .load::<(String, i32, chrono::NaiveDateTime)>(&req.state().connect().unwrap())
            .expect("Error loading session");

        if session.len() == 0 {
            return Ok(json!( {
                "error" : true,
                "error_msg" : "Failed to find user session"
            } ));
        }
        
        let (_, user, _) = session[0];
        
        /*use schema::users::dsl::*;
        let results = users.filter(username.eq(user.username))
            .limit(1)
            .load::<User>(&req.state().connect().unwrap())
            .expect("Error loading post");*/
        
        use schema::privillege::dsl::*;
        let privilleges = privillege
            .filter(groupname.eq(group_name)
                .and(schema::privillege::user_id.eq(user))) 
            .limit(1)
            .load::<(i32, String, String)>(&req.state().connect().unwrap())
            .expect("Error loading privilleges");
        
        if privilleges.len() == 0 {
            return Ok(json!( {
                "error" : true,
                "error_msg" : "You do not have any privilleges to access this group"
            } ));
        }
        
        let (_, _, user_rights) = &privilleges[0];
        let user_rights = user_rights.as_bytes();

        if user_rights[0] == b'_' {
            Ok(json!({
                "error" : true,
                "error_msg" : "You do not have any privilleges to access this group"
            }))
        } else {
            use schema::content::dsl::*;
            let loaded_content = content
                .filter(groupname.eq(group_name).and(page.eq(page_name)))
                .limit(1)
                .load::<(Option<String>, String, Option<String>)>(&req.state().connect().unwrap())
                .expect("Failed to load content");

            if loaded_content.len() == 0 {
                Ok(json!({
                    "error" : true,
                    "error_msg" : "Content not found"
                }))
            } else {
                let (_, _, loaded_content) = &loaded_content[0];
                let loaded_content = if let Some(x) = loaded_content {
                    x
                } else {
                    ""
                };
                Ok(json!({
                    "error" : false,
                    "error_msg" : "Content loaded properly",
                    "content" : loaded_content
                }))
            }
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
   
    app.at("/add-group").post(|mut req: tide::Request<DatabaseServer>| async move {
        let group: Group = req.body_form().await?;
        println!("Adding group: {}", group.name);
        use schema::groups::dsl::*;
        let results = diesel::insert_into(schema::groups::table)
            .values(group)
            .get_result::<Group>(&req.state().connect().unwrap())
            .expect("Failed to add group");

        println!("Added group: {:?}", results);

        Ok("Added group")
    });

    app.at("/add-privillege").post(|mut req: tide::Request<DatabaseServer>| async move {
        let mut added_privillege: PrivillegeByUsername = req.body_form().await?;
        added_privillege.username = added_privillege.username.to_lowercase();

        use schema::users::dsl::*;
        let results = users.filter(username.eq(added_privillege.username))
            .limit(1)
            .load::<User>(&req.state().connect().unwrap())
            .expect("Error loading post");
       
        if results.len() == 0 {
            return Ok("No users to add privillege");
        }
        
        let added_privillege: Privillege = Privillege {
            user_id: results[0].id,
            groupname: added_privillege.groupname,
            rights: added_privillege.rights,
        };

        let result = diesel::insert_into(schema::privillege::table)
            .values(added_privillege)
            .get_result::<Privillege>(&req.state().connect().unwrap())
            .expect("Failed to add privillege");

        println!("Added new privillege: {:?}", result);
        Ok("Added privillege")
    });

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

