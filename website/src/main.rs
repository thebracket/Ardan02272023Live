#[macro_use] extern crate rocket;
use auth::LoginAction;
use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[get("/")]
pub async fn index() -> NamedFile {
    NamedFile::open("login.html").await.unwrap()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginUser {
    username: String,
    password: String,
}

#[post("/api/login", data = "<user>")]
pub async fn login(user: Json<LoginUser>) -> Json<Option<LoginAction>> {
    use rocket::tokio::io::{AsyncWriteExt, AsyncReadExt};
    use rocket::tokio::net::TcpStream;
    
    let login_attempt = user.0;

    let mut stream = TcpStream::connect("127.0.0.1:8123").await.unwrap();
    let message = bincode::serialize(&login_attempt).unwrap();
    stream.write_all(&message).await.unwrap();

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response: Option<LoginAction> = bincode::deserialize(&buf[0..n]).unwrap();

    println!("{response:?}");
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login])
}