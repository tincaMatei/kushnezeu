extern crate diesel;
extern crate backend;
#[macro_use]
extern crate diesel_migrations;

use self::backend::*;
use self::models::*;
use self::database::DatabaseServer;

use tide::prelude::*;

pub mod database;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let server = DatabaseServer::start_database();
    
    server.run_migrations();
    
    let mut app = tide::with_state(server);
   
    app.at("/login").post(|mut req: tide::Request<DatabaseServer>| async move {
        let mut user: backend::models::User = req.body_form().await?;
        
        user.username = user.username.to_lowercase();
        let user_from_db = req.state().get_user_by_username(user.username);
        
        // We found an entry in the database of the given user
        if let Some(user_from_db) = user_from_db {
            if user_from_db.password == user.password { // The password matches
                let new_session = req.state().create_new_session(user_from_db.id);
                
                if let Some(new_session) = new_session {
                    println!("Expire date: {}", new_session.expire.to_string());
                    Ok(json!( {
                        "error" : false,
                        "error_msg" : "Login successful",
                        "session_id" : new_session.session_id,
                        "expire" : new_session.expire.to_string()
                    } ))
                } else {
                    Ok(json!( {
                        "error" : true,
                        "error_msg" : "Database error: Failed to insert session into database",
                        "session_id" : "",
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
        let session: Session = req.body_form().await?;
        println!("Session_id: {}", session.session_id);
        
        req.state().delete_session(session.session_id);
        
        Ok("Erased session")
    });

    app.at("/refresh-session").post(|mut req: tide::Request<DatabaseServer>| async move {
        let session: Session = req.body_form().await?;
        
        let expire_date = chrono::offset::Utc::now().naive_utc()
            + chrono::Duration::hours(12);
        
        req.state().set_session_expire_date(session.session_id, expire_date);

        Ok(json!( { "expire" : expire_date.to_string() } ))
    });

    // Return the content from the page
    app.at("/content/:group/:page/read").post(|mut req: tide::Request<DatabaseServer>| async move {
        let user_session: Session = req.body_form().await?;
        
        let group_name = req.param("group").expect("Failed to get group name");
        let page_name = req.param("page").expect("Failed to get page name");

        let session = req.state().get_session_by_id(user_session.session_id);
        let session = if let Some(x) = session {
            x
        } else {
            return Ok(json!({
                "error" : true,
                "error_msg" : "User not found"
            }));
        };
        
        let privillege = req.state().get_privillege(session.user_id, group_name.to_string());
        let privillege = if let Some(x) = privillege {
            x
        } else {
            return Ok(json!({
                "error" : true,
                "error_msg" : "You do not have any privillege to watch this"
            }));
        };

        if privillege.rights.as_bytes()[0] != b'R' {
            Ok(json!({
                "error" : true,
                "error_msg" : "You do not have any privilleges to access this group"
            }))
        } else {
            let content = req.state().get_content(group_name.to_string(), page_name.to_string());
            let content = if let Some(content) = content {
                content
            } else {
                return Ok(json!({
                    "error" : false,
                    "error_msg" : "This page is empty",
                    "content" : "This page is empty"
                }))
            };

            Ok(json!({
                "error" : false,
                "error_msg" : "Content loaded properly",
                "content" : content.contentbody
            }))
        }
    });

    app.at("/content/:group/:page/write").post(|mut req: tide::Request<DatabaseServer>| async move {
        let sent_data: SessionContentPost = req.body_form().await?;
        
        let group_name = req.param("group").expect("Failed to parse group name");
        let page_name = req.param("page").expect("Failed to parse page name");
        
        println!("{}", sent_data.session_id);
        let session = req.state().get_session_by_id(sent_data.session_id);
        println!("{:?}", session);
        let session = if let Some(x) = session {
            x
        } else {
            return Ok(json!({
                "error" : true,
                "error_msg" : "Failed to find session"
            }));
        };

        let privillege = req.state().get_privillege(session.user_id, group_name.to_string());
        let privillege = if let Some(x) = privillege {
            x
        } else {
            return Ok(json!({
                "error" : true,
                "error_msg" : "You do not have permission to write on this page"
            }));
        };

        if privillege.rights.as_bytes()[1] != b'W' {
            return Ok(json!({
                "error" : true,
                "error_msg" : "You do not have permission to write on this page"
            }));
        }

        req.state().add_content(group_name.to_string(), page_name.to_string(), sent_data.content);

        Ok(json!({
            "error" : false,
            "error_msg" : "Page added"
        }))
    });

    app.at("/get-rights/:group").post(|mut req: tide::Request<DatabaseServer>| async move {
        let session: Session = req.body_form().await?;
        
        let group_name = req.param("group").expect("failed to parse group_name");
        
        let session = req.state().get_session_by_id(session.session_id);
        let session = if let Some(x) = session {
            x
        } else {
            return Ok(json!({
                "error" : true,
                "error_msg" : "Session not found",
                "rights" : "____"
            }));
        };

        let privillege = req.state().get_privillege(session.user_id, group_name.to_string());
        let privillege = if let Some(x) = privillege {
            x
        } else {
            return Ok(json!({
                "error" : true,
                "error_msg" : "You do not have any privilleges for the group",
                "rights" : "____"
            }));
        };

        Ok(json!({
            "error" : false,
            "error_msg" : "Rights loaded properly",
            "rights" : privillege.rights
        }))
    });

    app.at("/list-groups").post(|mut req: tide::Request<DatabaseServer>| async move {
        let session: Session = req.body_form().await?;
        
        let session = req.state().get_session_by_id(session.session_id);
        let session = if let Some(x) = session {
            x
        } else {
            return Ok(json!({
                "error" : true,
                "error_msg" : "session not found",
            }));
        };

        let res = req.state().get_available_groups(session.user_id);
        Ok(json!({
            "error" : false,
            "error_msg" : "Groups listed properly",
            "groups" : res
        }))
    });

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

