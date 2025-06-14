use ::std::collections::HashMap;
use std::{
    convert::From,
    fmt::{self, Display},
};

/// Represents a value in a query string, which can be either a single value or multiple values for the same key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>), // heap allocated array, dynamically growing
}

/// Represents a parsed query string, storing key-value pairs where both key and value are string slices from the same buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryString<'buf> {
    // both key and value comes from the same buffer, so they have the same lifetime as `buf`
    data: HashMap<&'buf str, Value<'buf>>,
}
impl<'buf> QueryString<'buf> {
    /// Retrieves the value associated with the given key from the query string.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up in the query string.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Value` if the key exists, or `None` otherwise.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

/// Implements conversion from a string slice to a `QueryString`.
///
/// # Note
///
/// The `From` trait is used instead of `TryFrom` because the conversion from a string to a `QueryString` cannot fail.
/// The input string buffer is assumed to have a valid format for query strings (e.g., `key1=value1&key2=value2`).
impl<'buf> From<&'buf str> for QueryString<'buf> {
    /// Converts a string slice into a QueryString by parsing key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice containing the query string to parse (e.g., "key1=value1&key2=value2")
    ///
    /// # Returns
    ///
    /// A new `QueryString` instance containing the parsed key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// let query = QueryString::from("name=John&age=30");
    /// ```
    fn from(s: &'buf str) -> Self {
        // Initialize an empty HashMap to store the key-value pairs
        let mut data = HashMap::new();

        // Split the input string by '&' to get individual key-value pairs
        for sub_str in s.split('&') {
            // Default values in case no '=' is found
            let mut key = sub_str;
            let mut value = "";

            // If '=' is found, split the substring into key and value
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }

            // Insert the key-value pair into the HashMap
            data.entry(key)
                // Handle the case where a key already exists in the HashMap
                .and_modify(|existing: &mut Value| match existing {
                    // If the key exists with a single value, convert it to a Multiple value
                    Value::Single(prev_value) => {
                        *existing = Value::Multiple(vec![prev_value, value]); // dereference the existing value and assign a new value
                    }
                    // If the key already has multiple values, append the new value
                    Value::Multiple(vec) => vec.push(value),
                })
                // If the key does not exist, insert a new single value
                .or_insert(Value::Single(value));
        }

        // Create and return a new QueryString with the parsed data
        QueryString { data }
    }
}

/// Implements the `Display` trait for `QueryString`, formatting it as a string of joined keys separated by '&'.
impl<'buf> Display for QueryString<'buf> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .keys()
                .map(|key| key.to_string())
                .collect::<Vec<String>>()
                .join("&")
        )
    }
}
