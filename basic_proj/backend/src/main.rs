use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;

#[macro_use]
extern crate serde_derive;

//user struct
#[derive(Serialize, Deserialize)]
struct User{
    id: Option<i32>,
    name: String,
    email: String,
}

//database
const DB_URL: &str = env!("DATABASE_URL");

//constants for cqrs
const OK_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

fn main(){
    //set db
    if let Err(_) = set_database(){
        print!("Error setting db");
        return;
    }

    // start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    print!("Server lis on 8080");

    for stream in listener.incoming()   {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                print!("Unable to connect: {}", e)
            }
        }
    }

}