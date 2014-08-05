//use std::str::from_utf8_owned;
use std::from_str::FromStr;

#[deriving(Clone)]
pub enum Error {
    ResponseError,
    ExecAbortError,
    BusyLoadingError,
    NoScriptError,
    UnknownError,
    ExtensionError(String),
}

#[deriving(Clone)]
pub enum Value {
    Invalid,
    Nil,
    Int(i64),
    Data(Vec<u8>),
    Bulk(Vec<Value>),
    Error(Error, String),
    Success,
    Status(String),
}

#[deriving(Clone)]
pub enum ConnectFailure {
    InvalidURI,
    HostNotFound,
    ConnectionRefused,
}

#[deriving(Clone)]
pub enum CmdArg<'a> {
    StrArg(&'a str),
    IntArg(i64),
    FloatArg(f32),
    BytesArg(&'a [u8]),
}

#[deriving(Clone)]
pub enum ShutdownMode {
    ShutdownNormal,
    ShutdownSave,
    ShutdownNoSave,
}

#[deriving(Clone)]
pub enum KeyType {
    StringType,
    ListType,
    SetType,
    ZSetType,
    HashType,
    UnknownType,
    NilType,
}

#[deriving(Clone)]
pub enum RangeBoundary {
    Open(f32),
    Closed(f32),
    Inf,
    NegInf,
}

pub trait ToStr {
    fn to_str(&self) -> String;
}

impl ToStr for RangeBoundary {
    fn to_str(&self) -> String {
        match *self {
            Open(x) => format!("({}", x),
            Closed(x) => format!("{}", x),
            Inf => "+inf".to_string(),
            NegInf => "-inf".to_string(),
        }
    }
}

impl Value {

    pub fn get_bytes(self) -> Option<Vec<u8>> {
        match self {
            Data(payload) => Some(payload),
            _ => None,
        }
    }

    pub fn get_string(self) -> Option<String> {
        match self {
            Status(x) => Some(x),
            Data(payload) => match String::from_utf8(payload) { Ok(x) => {Some(x)} _ => None },
            _ => None,
        }
    }

    pub fn get_as<T: FromStr>(self) -> Option<T> {
        match self.get_string() {
            Some(x) => from_str(x.as_slice()),
            None => None,
        }
    }
}
