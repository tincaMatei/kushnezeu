use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::result::ConnectionResult;
use diesel_migrations::*;
use super::*;

embed_migrations!("migrations");

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

#[derive(Clone)]
pub struct DatabaseServer {
    database_url: String,
}

impl DatabaseServer {
    pub fn start_database() -> DatabaseServer {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
       
        DatabaseServer {
            database_url,
        }
    }

    pub fn connect(&self) -> ConnectionResult<PgConnection> {
        PgConnection::establish(&self.database_url)
    }

    pub fn run_migrations(&self) {
        embedded_migrations::run(&self.connect().unwrap())
            .expect("Failed to run migrations");
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Option<User> {
        use schema::users::dsl::*;
        let mut results = users.filter(id.eq(user_id))
            .limit(1)
            .load::<User>(&self.connect().unwrap())
            .expect("Error loading user from database");
        
        results.pop()
    }

    pub fn get_user_by_username(&self, sent_username: String) -> Option<User> {
        use schema::users::dsl::*;
        let sent_username = sent_username.to_lowercase();
        let mut results = users.filter(username.eq(sent_username))
            .limit(1)
            .load::<User>(&self.connect().unwrap())
            .expect("Error loading user from database");
    
        results.pop()
    }

    pub fn create_new_session(&self, user_id: i32) -> Option<Session> {
        let token = lazy_token(64);

        let expire_date = chrono::offset::Utc::now().naive_utc()
            + chrono::Duration::hours(12);

        let new_session = Session {
            session_id: token,
            user_id,
            expire: expire_date,
        };

        use schema::sessions;
        let result = diesel::insert_into(sessions::table)
            .values(new_session)
            .get_result::<(String, i32, chrono::NaiveDateTime)>(&self.connect().unwrap());
    
        if let Ok((token, user_id, expire_date)) = result {
            Some(Session {
                session_id: token,
                user_id,
                expire: expire_date
            })
        } else {
            None
        }
    }
    
    // Delete all irrel
    pub fn delete_session(&self, target_session: String) {
        use backend::schema::sessions::dsl::*;
        let expire_date = chrono::offset::Utc::now().naive_utc();
        let _num_deleted = diesel::delete(sessions
            .filter(session_id.eq(&target_session)
                .or(&expire.le(expire_date))))
            .execute(&self.connect().unwrap())
            .expect("Failed to delete the session");
    }

    pub fn set_session_expire_date(&self, 
        target_session: String, 
        new_expire: chrono::NaiveDateTime) {
        
        use backend::schema::sessions::dsl::*;
        let updated = diesel::update(sessions.find(target_session))
            .set(expire.eq(new_expire))
            .get_result::<Session>(&self.connect().unwrap())
            .expect("Failed to modify session expire time");
        println!("Refreshed session: {:?}", updated);
    }

    pub fn get_session_by_id(&self, target_session: String) -> Option<Session>{
        use backend::schema::sessions::dsl::*;
        let mut session = sessions.filter(session_id.eq(target_session))
            .limit(1)
            .load::<Session>(&self.connect().unwrap())
            .expect("Error loading session");

        session.pop()
    }

    pub fn get_privillege(&self, target_user: i32, target_group: String) -> Option<Privillege> {
        use schema::privillege::dsl::*;
        let mut privilleges = privillege
            .filter(groupname.eq(target_group)
                .and(schema::privillege::user_id.eq(target_user))) 
            .limit(1)
            .load::<Privillege>(&self.connect().unwrap())
            .expect("Error loading privilleges");
        
        privilleges.pop()
    }

    pub fn get_content(&self, group_name: String, page_name: String) -> Option<Content>{
        use schema::content::dsl::*;
        let mut loaded_content = content
            .filter(groupname.eq(group_name).and(page.eq(page_name)))
            .limit(1)
            .load::<Content>(&self.connect().unwrap())
            .expect("Failed to load content");
    
        loaded_content.pop()
    }

    pub fn add_group(&self, group_name: &Group) {
        let _results = diesel::insert_into(schema::groups::table)
            .values(group_name)
            .get_result::<Group>(&self.connect().unwrap())
            .expect("Failed to add group");
    }

    pub fn add_account(&self, new_user: &NewUser) {
        use schema::users;
        let _result = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(&self.connect().unwrap())
            .expect("Error creating new user");
    }

    pub fn add_privillege(&self, target_user: i32, target_group: String, rights: String) {
        let added_privillege = Privillege {
            user_id: target_user,
            groupname: target_group,
            rights
        };
    
        let _result = diesel::insert_into(schema::privillege::table)
            .values(added_privillege)
            .get_result::<Privillege>(&self.connect().unwrap())
            .expect("Failed to add privillege");
    }

    pub fn add_content(&self, group_name: String, page_name: String, added_content: String) {
        let content_page = Content {
            groupname: Some(group_name),
            page: page_name,
            contentbody: Some(added_content)
        };
        
        let _result = diesel::insert_into(schema::content::table)
            .values(content_page)
            .get_result::<Content>(&self.connect().unwrap())
            .expect("Failed to add content");
    }
}
