use local_ip_address::local_ip;

#[tokio::main]
async fn main() {
    let local = local_ip().expect("Can't get local IP address");
    println!("Local IP Address: {:?}", local);
    if let Some(ip) = public_ip::addr().await {
        println!("Public IP Address: {:?}", ip);
    } else {
        println!("Can't get public IP address");
    }
}
