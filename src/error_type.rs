use std::error::{Error};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ET {
    OK,
    TODO,
    RaftEmptyLogEntry,
    RaftIndexOutOfRange,
    RaftCannotFindId,
    ChSendError,
    ChRecvError,
    ErrorLength,
    ErrorType,
    ExistingSuchKey,
    ExistingSuchElement,
    NoSuchKey,
    NoSuchElement,
    NoneOption,
    ExceedCapacity,
    OutOffIndex,
    CorruptLog,
    SenderError(String),
    RecvError(String),
    TokioSenderError(String),
    TokioRecvError(String),
    EOF,
    IOError(String),
    JSONError(String),
    SerdeError(String),
    CSVError(String),
    ParseError(String),
    TSParseError(String),
    FatalError(String),
    TxConflict,
    ErrorCursor,
    Deadlock,
    StopService,
    NetNotConnected,
}

unsafe impl Send for ET {}

impl fmt::Display for ET {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ET {

}