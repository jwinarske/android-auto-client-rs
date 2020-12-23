use protobuf_codegen_pure::Customize;

fn main() {

    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir("src")
        .input("src/protos/android.auto.proto")
        .include("src/protos")
        .run()
        .expect("protoc");
}