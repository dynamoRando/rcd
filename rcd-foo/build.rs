use std::io;

fn main() -> io::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("./src")
        .compile(&["rcdp.proto"], &["."])
}