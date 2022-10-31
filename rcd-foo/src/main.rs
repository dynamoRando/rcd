use rcdproto::rcdp::TestRequest;

fn main() {
    let test = TestRequest {
        request_time_utc: "".to_string(),
        request_origin_url: "".to_string(),
        request_origin_ip4: "".to_string(),
        request_origin_ip6: "".to_string(),
        request_port_number: 0,
        request_echo_message: "hello".to_string(),
    };

    let output = serde_json::to_string(&test);

    println!("Hello, world!");
    println!("{}", output.unwrap());
}
