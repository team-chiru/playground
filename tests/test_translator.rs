#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

#[cfg(feature = "serde_macros")]
include!("translate.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/translate.rs"));

extern crate serde_test;
use serde_test::{Token, assert_tokens};

#[test]
fn test_translator_serde() {
    let mut pack: HashMap<String, String> = HashMap::new();
    pack.insert(String::from("EN"), String::from("World"));

    let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
    map.insert(String::from("Hello"), pack);
    let translator = Translator::new(map);

    assert_tokens(&translator, &[
        Token::StructStart("Translator", 1),
        Token::StructSep,
            Token::Str("data"),
            Token::MapStart(Some(1)),
            Token::MapSep,
                Token::Str("Hello"),
                Token::MapStart(Some(1)),
                Token::MapSep,
                    Token::Str("EN"),
                    Token::Str("World"),
                Token::MapEnd,
            Token::MapEnd,
        Token::StructEnd,
    ]);
}
