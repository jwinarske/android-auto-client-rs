use std::env;

fn main() {
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();
        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=protos/android_auto.proto");
    prost_build::compile_protos(&["protos/android_auto.proto"],
                                &["protos/"]).unwrap();
}
