mod codec;
    use crate::codec::RespCodec;
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use std::sync::Mutex;
    use tokio::net::TcpListener;
    use tokio::net::TcpStream;
    use tokio::prelude::*;
    use tokio_codec::Decoder;
    use std::env;






fn main() {
    println!("Hello, world!");
}
