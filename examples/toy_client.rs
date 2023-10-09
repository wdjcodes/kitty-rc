use std::env;

use kitty_rc::client::Client;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    println!("{}", path);
    let mut client = Client::connect(path).unwrap();

    let command = b"{\"cmd\":\"ls\",\"version\":[0,26,5]}";
    let bytes = client.send_command(command).unwrap();
    println!("Sent {bytes} bytes");
    
    let response;

    response = client.get_response().unwrap();
    println!("{}", response.data());
}