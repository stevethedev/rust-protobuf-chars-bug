use protoc_rust::Customize;

fn main() {
  protoc_rust::run(protoc_rust::Args {
    out_dir: "src",
    input: &["proto/user.proto"],
    includes: &["proto"],
    customize: Customize {
      serde_derive: Some(true),
      carllerche_bytes_for_bytes: Some(true),
      carllerche_bytes_for_string: Some(true),
      ..Default::default()
    },
  })
  .expect("protoc");
}
