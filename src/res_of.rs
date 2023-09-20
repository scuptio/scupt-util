use crate::error_type::ET;
use crate::res::Res;

pub fn res_sqlite<T>(r: rusqlite::Result<T>) -> Res<T> {
    match r {
        Ok(t) => { Ok(t) }
        Err(e) => { return Err(ET::IOError(e.to_string())); }
    }
}

pub fn res_io<T>(r: Result<T, std::io::Error>) -> Res<T> {
    match r {
        Ok(t) => Ok(t),
        Err(e) => Err(ET::IOError(e.to_string())),
    }
}

pub fn res_parse<T, E: ToString>(r: Result<T, E>) -> Res<T> {
    match r {
        Ok(t) => Ok(t),
        Err(e) => Err(ET::ParseError(e.to_string())),
    }
}

pub fn res_option<T>(opt: Option<T>) -> Res<T> {
    match opt {
        Some(t) => Ok(t),
        None => Err(ET::NoneOption),
    }
}

