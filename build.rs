
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=protos/android_auto.proto");
    prost_build::compile_protos(&["protos/android_auto.proto"],
                                &["protos/"]).unwrap();
}
