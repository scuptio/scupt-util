#![feature(const_trait_impl)]

pub mod error_type;
pub mod id;
pub mod res;
pub mod escape_string;
pub mod init_logger;
pub mod fn_compare;
pub mod fn_hash;
pub mod slice;
pub mod fn_key;
pub mod datum;
pub mod stub_datum;
pub mod message;
pub mod node_id;
pub mod datum_msg;
pub mod get_exec;
pub mod res_of;
pub mod backtrace;
pub mod ts_node_context_string;
pub mod logger;
pub mod sj_value_ref;
pub mod mt_map;
pub mod mt_set;
mod test_compare_sj_value;
mod test_mt_set;
mod test_mt_map;
