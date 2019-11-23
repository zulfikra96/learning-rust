pub mod controllers;

extern crate actix_web;
extern crate postgres;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use postgres::{Connection, TlsMode};
use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse};

#[derive(Serialize, Deserialize)]
struct User {
    users_id:i32,
    username:String,
    fullname:String,
    password:String
}

fn pg_connection() -> Connection {
    let conn = Connection::connect("postgresql://postgres:Billgates1996@localhost:5432/rust",TlsMode::None)
        .unwrap();
    return conn;
}

fn index(req: HttpRequest) -> HttpResponse {
    let mut rows: Vec<User> = vec![];
    for row in &pg_connection().query("SELECT * FROM users",&[]).unwrap()
    {
        let person = User {
            users_id:row.get(0),
            username:row.get(1),
            fullname:row.get(2),
            password:row.get(3)
        };
        rows.push(person)
    }
    // println!("{:?}", rows);
    // controllers::home::index();
    return HttpResponse::Ok().json(rows);
}

fn main() {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}