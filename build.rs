fn main() {
    tonic_build::compile_protos("./proto/TemplatesService.proto").unwrap();
    tonic_build::compile_protos("./proto/SecretsService.proto").unwrap();
}
