use libafl:: {
    generators::NautilusContext,
    inputs::NautilusInput,
};
use regex::Regex;

// 基于libafl的 Nautilus 实现 grammar
pub fn get_trackmania_context(tree_depth: usize) -> NautilusContext {
    let mut str_grammar = Vec::new();
    // ?? 
    macro_rules! add_rule {
        ($a:expr, $b:expr) => {
            str_grammar.push([$a.to_string(), $b.to_string()].to_vec())
        };
    }

    add_rule!(
        "RPC-CALL",
        "<?xml version=\"1.0\"?><methodCall>{METHOD_CALL}</methodCall>"
    );

    let method_re = Regex::new(r"\w+(.*)\((.*))\)").unwrap();

    NautilusContext::new(tree_depth, &str_grammar)
}