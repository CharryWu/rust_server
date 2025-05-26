use super::method::Method;
pub struct Request {
    method: Method,
    query_string: Option<String>,
    path: String,
}
