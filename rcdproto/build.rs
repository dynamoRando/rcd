use std::{env, path::PathBuf};

fn main() {

    println!("build main");

    let proto_file = "./proto/rcdp.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
   
    println!("{}", out_dir.display());

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("greeter_descriptor.bin"))
        .out_dir("./src")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
