use std::path::PathBuf;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::Connection;

use crate::error_type::ET;
use crate::res::Res;
use crate::res_of::res_sqlite;

pub fn get_f_exec_by_id(
    path: String,
    action_name: String,
    ids: Vec<String>,
) -> Res<Vec<(String, String)>> {
    let path_buf = PathBuf::from(path);
    let path_buf = path_buf.join(format!("{}.sqlite", action_name));
    let r_conn = Connection::open(path_buf);
    let conn = res_sqlite(r_conn)?;
    let ret = conn_get_exec_by_id(conn, ids)?;
    Ok(ret)
}

pub fn get_f_exec_rand(
    path: String,
    action_name: String,
    amount: usize,
) -> Res<Vec<(String, String)>> {
    let path_buf = PathBuf::from(path);
    let path_buf = path_buf.join(format!("{}.sqlite", action_name));
    let r_conn = Connection::open(path_buf);
    let conn = res_sqlite(r_conn)?;
    let mut id_choose = vec![];
    {
        let r = conn.prepare("SELECT id FROM execution");
        let mut stmt = res_sqlite(r)?;
        let r = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            Ok(id)
        });
        let exec_iter = res_sqlite(r)?;
        let mut ids = vec![];
        for r_id in exec_iter {
            match r_id {
                Ok(id) => {
                    ids.push(id);
                }
                Err(e) => {
                    return Err(ET::IOError(e.to_string()));
                }
            }
        }

        for id in ids.choose_multiple(&mut thread_rng(), amount) {
            id_choose.push(id.clone())
        }
    }
    let ret = conn_get_exec_by_id(conn, id_choose)?;
    Ok(ret)
}


pub fn conn_get_exec_by_id(
    conn: Connection,
    ids: Vec<String>, ) -> Res<Vec<(String, String)>> {
    let mut ret = vec![];
    let r = conn.prepare("SELECT exec FROM execution where id = ?");
    let mut stmt = res_sqlite(r)?;
    for id in ids.iter() {
        let r = stmt.query_map([id.clone()], |row| {
            let json: String = row.get(0)?;
            Ok(json)
        });
        let exec_iter = res_sqlite(r)?;

        let json = match exec_iter.last() {
            Some(r) => {
                match r {
                    Ok(j) => {
                        j
                    }
                    Err(e) => {
                        return Err(ET::IOError(e.to_string()));
                    }
                }
            }
            None => {
                return Err(ET::NoneOption);
            }
        };
        ret.push((id.clone(), json));
    }
    Ok(ret)
}