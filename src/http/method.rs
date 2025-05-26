pub enum Method {
    GET(Option<String>), // associated with query string
    POST,
    PUT,
    DELETE(u64), // associated with user id
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
