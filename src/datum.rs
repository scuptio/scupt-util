use crate::slice::Slice;

pub trait Datum: Slice + Clone {}