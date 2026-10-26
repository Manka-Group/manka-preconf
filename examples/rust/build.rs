fn main() {
    tonic_build::compile_protos("../../proto/sugar.proto").unwrap();
}
