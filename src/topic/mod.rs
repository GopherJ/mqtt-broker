const TOPIC_SEPARATOR: char = '/';

const MULTI_LEVEL_WILDCARD: char = '#';
const MULTI_LEVEL_WILDCARD_STR: &str = "#";

const SINGLE_LEVEL_WILDCARD: char = '+';
const SINGLE_LEVEL_WILDCARD_STR: &str = "+";

const SHARED_SUBSCRIPTION_PREFIX: &str = "$share/";

pub const MAX_TOPIC_LEN_BYTES: usize = 65_535;

mod filter;
mod tree;

pub use filter::*;
pub use tree::*;