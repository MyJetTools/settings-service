fn main() {
    ci_utils::ProtoFileBuilder::new("../proto-files/")
        .sync_and_build("TemplatesService.proto")
        .sync_and_build("SecretsService.proto");
}
