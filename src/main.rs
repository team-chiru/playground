// serde nightly stuff (not really clean)
#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

#[cfg(feature = "serde_macros")]
include!("translate.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/translate.rs"));

extern crate rand;
extern crate yaml_rust;

#[macro_use]
extern crate clap;

use std::io;
use std::io::Read;
use std::cmp::Ordering;
use rand::Rng;
use clap::App;

const TRADUCTION_PATH: &'static str = "res/lang.yaml";

// maybe store it into db (or better: into cache)
static mut LANG: Lang = Lang::EN;

// macro for translate: it's really dumb and not optimize
macro_rules! translate {
    ($msg:expr) => {{
        // i need to call translator constructor each time...
        // it's maybe better to load some critic file in the cache
        // TODO test redis-rs
        let translator = Translator::new_from_config(TRADUCTION_PATH);
        unsafe {
            match translator.translate($msg, &LANG) {
                Some(s) => println!("{}", s),
                _ => println!("{}", $msg),
            }
        }
    }};
}

fn main() {
    // load cli configuration
    // cli data scheme : cli.yaml
    let yaml_cli = load_yaml!("../res/cli.yaml");
    let matches = App::from_yaml(yaml_cli).get_matches();

    let lang_cli = match matches.value_of("lang") {
        Some(l) => l,
        _ => "en",
    };

    // load language configuration
    unsafe {
        LANG = Lang::from(lang_cli);
    }

    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(..) => {},
            Err(_) => println!("Failed to read line"),
        }

        match guessing_game(&guess, &secret) {
            GResult::BadParsing => continue,
            GResult::Quit => break,
            GResult::Guess(r) => {
                answer(r);
                match r {
                    Ordering::Equal => break,
                    _ => continue,
                }
            },
        };
    }
}

enum GResult {
    Quit,
    BadParsing,
    Guess(Ordering),
}

fn answer(a: Ordering) {
    match a {
        Ordering::Less => translate!("less"),
        Ordering::Greater => translate!("greater"),
        Ordering::Equal => translate!("equal"),
    }
}

fn guessing_game(guess: &String, secret: &u32) -> GResult {
    if guess.trim().eq("quit") {
        return GResult::Quit;
    }

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return GResult::BadParsing;
        },
    };

    println!("You guessed: {}", guess);
    GResult::Guess(guess.cmp(&secret))
}
