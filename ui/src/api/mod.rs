pub mod envs;
pub mod products;
pub mod secrets;
pub mod templates;

pub(crate) fn base_url() -> String {
    let origin = web_sys::window()
        .expect("no window")
        .location()
        .origin()
        .expect("no origin");
    if origin.ends_with('/') {
        origin[..origin.len() - 1].to_string()
    } else {
        origin
    }
}
