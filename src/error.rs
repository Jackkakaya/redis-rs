#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Resp,

    KeyNotFound,
    RedisError(String),
    ParseRedisReply(String),
    ConnectionPoolClosed,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::Resp => write!(f, "wrong Redis serialization protocol format"),

            Error::KeyNotFound => write!(f, "key not found"),
            Error::RedisError(s) => write!(f, "{}", s),
            Error::ConnectionPoolClosed => write!(f, "connection pool closed"),
            Error::ParseRedisReply(s) => write!(f, "parse redis failed: {}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseRedisReply(format!("{}", e))
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseRedisReply(format!("{}", e))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::ParseRedisReply(format!("{}", e))
    }
}
