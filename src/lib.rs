extern crate rdp;

use neon::prelude::*;
use std::net::{SocketAddr, TcpStream};
use rdp::core::client::Connector;


pub fn do_check_credentials(username: &str, password: &str, host: &str) -> bool {
    let host = match host.parse::<SocketAddr>() {
        Ok(host) => host,
        Err(_) => return false,
    };
    let tcp = TcpStream::connect(host);
    let tcp = match tcp {
        Ok(tcp) => tcp,
        Err(_) => return false,
    };
    let mut connector = Connector::new()
        .screen(800, 600)
        .auto_logon(true)
        .check_certificate(false)
        .credentials(host.to_string(), username.to_string(), password.to_string());

    match connector.connect(tcp) {
        Ok(_) => {
            dbg!("Connected!");
            true
        }
        Err(_) => {
            dbg!("Failed to connect!");
            false
        },
    }
}

fn check_credentials(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let username = cx.argument::<JsString>(0).unwrap().value(&mut cx);
    let password = cx.argument::<JsString>(1).unwrap().value(&mut cx);
    let host = cx.argument::<JsString>(2).unwrap().value(&mut cx);
    let result = do_check_credentials(&username, &password, &host);
    Ok(cx.boolean(result))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("check_credentials", check_credentials)?;
    Ok(())
}


