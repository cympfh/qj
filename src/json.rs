#[derive(Debug)]
pub enum Json {
    Int(i64),
    Float(f64),
    Array(Vec<Box<Json>>),
    Map(Vec<(String, Box<Json>)>),
}
