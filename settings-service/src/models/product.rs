use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Clone)]
pub struct Product {
    pub id: String,
    pub description: String,
    pub prompt: String,
    pub created: DateTimeAsMicroseconds,
    pub updated: DateTimeAsMicroseconds,
}
