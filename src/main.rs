mod rcd;

fn main() {
    println!("Hello, world!");
    rcd::hello();
    let service = rcd::get_service_from_config_file();
    println!("{:?}", service);
}
