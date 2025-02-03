use arcstr::ArcStr;
use phf::{phf_set, Set};

pub(crate) const CONFIGURATION_LIT: &'static str = "configuration";
pub(crate) const PARAMETER_LIT: &'static str = "parameter";
pub(crate) const OVERRIDE_LIT: &'static str = "override";
pub(crate) const LOCALS_LIT: &'static str = "locals";

// FIXME: We should use the same token for declaring and accessing local variables
pub(crate) const LOCAL_LIT: &'static str = "local";
pub(crate) const EXTEND_LIT: &'static str = "extends";

pub(crate) const OTHER_EXTEND_CONFIGURATION_PARENT_ID: &'static str = "moss.configuration.other";

// FIXME: Apparently phf_set macro only supports literals
pub(crate) static RESERVED_WORDS: Set<&'static str> = phf_set! {
    "configuration",
    "parameter",
    "override",
    "locals",
    "local",
    "extends",
    "moss.configuration.other"
};
