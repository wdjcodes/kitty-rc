use std::env;

use kitty_rc::client::Client;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    println!("{}", path);
    let mut client = Client::connect(path).unwrap();

    let command = b"{\"cmd\":\"get-colors\",\"version\":[0,26,5]}";
    let mut bytes = client.send_command(command).unwrap();
    println!("Sent {bytes} bytes");
    
    let mut response = [0u8; 15000];

    bytes = client.get_response(&mut response).unwrap();
    println!("{}", String::from_utf8_lossy(&response).to_string());
    println!("Received {bytes} bytes");
}