
fn main() {
    prost_build::compile_protos(&["protos/android.auto.proto"],
                                &["protos/"]).unwrap();
}
