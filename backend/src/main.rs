extern crate diesel;
extern crate backend;

fn main() {
    let connection = backend::establish_connection();

    backend::add_user(&connection, "Mirel", "1234");
}

//use tide::Request;
//use tide::prelude::;

//#[async_std::main]
//async fn main() -> tide::Result<()> {
//    let mut app = tide::new();

//    app.at("/*").get(|req| async move { Ok("Thanks, Kanye, very cool!") });
//    app.listen("0.0.0.0:8080").await?;
//    Ok(())
//}

