use std::str::FromStr;

pub enum Method {
    GET, // (Option<String>), // associated with query string
    POST,
    PUT,
    DELETE, // (u64), // associated with user id
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

pub struct MethodError {
    message: String,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0].to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(MethodError {
                message: "Invalid HTTP method".to_string(),
            }),
        }
    }
}
