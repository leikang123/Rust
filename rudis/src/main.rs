
use std :: thread;
use std :: env;
use std :: sync::Mutex;
use std :: collections::HashMap;
use std ::modules::commands;
use std :: io ::{BufReader,Write};
use std :: net :: Shutdown;
use std :: net ::{TcpListener,TcpStream};
use resp :: Decoder;
use lazy_static::lazy_static;
mod commands;
    use crate::commands::process_client_request;
type STORE = Mutex<HashMap<String, String>>;
lazy_static! {
static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}
fn main() {
let addr = env::args()
.skip(1)
.next() .unwrap_or("127.0.0.1:6378".to_owned());
let listener = TcpListener::bind(&addr).unwrap(); println!("rudis_sync listening on {} ...", addr);
for stream in listener.incoming() {
let stream = stream.unwrap();
println!("New connection from: {:?}", stream); handle_client(stream);

}

}
fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    let decoder = Decoder::new(&mut stream).decode(); match decoder {
    Ok(v) => {
    let reply = process_client_request(v); stream.get_mut().write_all(&reply).unwrap();
    }
    Err(e) => {
    println!("Invalid command: {:?}", e);
    let _ = stream.get_mut().shutdown(Shutdown::Both); }
    }; 
}

pub fn process_client_request(decoded_msg: Value) -> Vec<u8> {
    let reply = if let Value::Array(v) = decoded_msg { match &v[0] {
        Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
        Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
        other => unimplemented!("{:?} is not supported as of now",
        other), }
        } else {
        Err(Value::Error("Invalid Command".to_string()))
        };
        match reply {
        Ok(r) | Err(r) => r.encode(),
        } 
    }

    pub fn handle_get(v: Vec<Value>) -> Result<Value, Value> { let v = v.iter().skip(1).collect::<Vec<_>>();
        if v.is_empty() { 
            return Err(Value::Error("Expected 1 argument for GET command".to_string()))
}
let db_ref = RUDIS_DB.lock().unwrap();
let reply = if let Value::Bulk(ref s) = &v[0] {
db_ref.get(s).map(|e| Value::Bulk(e.to_string())).unwrap_or(Value::Null)
        } else {
            Value::Null
};
Ok(reply) }
pub fn handle_set(v: Vec<Value>) -> Result<Value, Value> { let v = v.iter().skip(1).collect::<Vec<_>>();
if v.is_empty() || v.len() < 2 {
return Err(Value::Error("Expected 2 arguments for SET command".to_string()))
        }
        match (&v[0], &v[1]) {
(Value::Bulk(k), Value::Bulk(v)) => { let _ = RUDIS_DB
.lock()
.unwrap()
.insert(k.to_string(), v.to_string());
}
_ => unimplemented!("SET not implemented for {:?}", v), }
Ok(Value::String("OK".to_string()))
 }