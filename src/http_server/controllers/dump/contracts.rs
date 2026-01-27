use my_http_server::{macros::MyHttpInput, types::FileContent};

#[derive(MyHttpInput)]
pub struct ImportSettingsTemplateAction {
    #[http_query(description:"Product")]
    pub product: String,
    #[http_form_data(name = "dump", description = "Dump file")]
    pub dump: FileContent,
}

#[derive(MyHttpInput)]
pub struct ImportSnapshotModel {
    #[http_query(description:"Product")]
    pub product: String,
    #[http_form_data(name = "snapshot", description = "Snapshot of templates and secrets")]
    pub dump: FileContent,
}

#[derive(Debug, MyHttpInput)]
pub struct ExportSnapshotHttpInputData {
    #[http_query(description:"Product")]
    pub product: String,
}
