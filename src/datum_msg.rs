use crate::datum::Datum;
use crate::message::MsgTrait;

pub trait DatumMsg: Datum + MsgTrait {}