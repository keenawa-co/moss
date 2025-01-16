use phf::phf_set;

// TODO

static RESERVED_WORDS: phf::Set<&'static str> = phf_set! {
    "configuration",
    "theme",
};

fn is_reserved_word(value: &str) -> bool {
    RESERVED_WORDS.get_key(value).is_some()
}

fn is_valid_configuration_id(value: &str) -> bool {
    unimplemented!()
}

pub struct Parser {}
