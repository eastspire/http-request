#[cfg(test)]
mod cfg;

pub(crate) mod body;
pub(crate) mod common;
pub(crate) mod r#const;
pub(crate) mod global_trait;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod utils;

pub use http_type::{
    HashMapXxHash3_64, JsonDeserializer, JsonError, JsonMap, JsonNumber, JsonResult,
    JsonSerializer, JsonStreamDeserializer, JsonValue, hash_map_xx_hash3_64, json_from_reader,
    json_from_slice, json_from_str, json_from_value, json_to_string, json_to_string_pretty,
    json_to_value, json_to_vec, json_to_vec_pretty, json_to_writer, json_to_writer_pretty,
    json_value,
};
pub use request::*;
pub use response::*;

pub(crate) use body::*;
pub(crate) use common::*;
pub(crate) use r#const::*;
pub(crate) use global_trait::*;
pub(crate) use utils::vec::*;

pub(crate) use http_type::{
    ACCEPT, ACCEPT_ANY, BR_BYTES, COLON_SPACE_BYTES, CONTENT_LENGTH, CONTENT_TYPE, Compress,
    ContentType, DEFAULT_BUFFER_SIZE, DEFAULT_HTTP_PATH, DEFAULT_MAX_REDIRECT_TIMES,
    DEFAULT_TIMEOUT, EMPTY_STR, HOST, HTTP_BR, HTTP_BR_BYTES, HTTP_DOUBLE_BR_BYTES, HttpStatus,
    HttpUrlComponents, HttpVersion, LOCATION, Method, Protocol, QUERY_SYMBOL, RequestBody,
    RequestBodyString, RequestError, RequestHeaders, ResponseHeaders, ResponseStatusCode, SPACE_U8,
    TAB_U8, USER_AGENT, ZERO_STR,
};
pub(crate) use rustls::{
    ClientConfig, ClientConnection, RootCertStore, StreamOwned, pki_types::ServerName,
};
pub(crate) use serde::{Serialize, Serializer};
pub(crate) use std::{
    collections::HashSet,
    fmt::{self, Debug, Display},
    net::TcpStream,
    sync::{Arc, RwLock},
    time::Duration,
    vec::IntoIter,
};
