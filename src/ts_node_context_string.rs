use tree_sitter::Node;

use crate::error_type::ET;
use crate::res::Res;

pub fn ts_node_context_string(text: &String, node: &Node) -> Res<String> {
    let range = node.range();
    let r = String::from_utf8(
        text.as_bytes()[range.start_byte..range.end_byte].to_vec()
    );
    match r {
        Ok(s) => Ok(s),
        Err(e) => Err(ET::TSParseError(e.to_string()))
    }
}