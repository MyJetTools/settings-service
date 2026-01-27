fn main() {
    tonic_prost_build::compile_protos("./proto/TemplatesService.proto").unwrap();
    tonic_prost_build::compile_protos("./proto/SecretsService.proto").unwrap();
}
