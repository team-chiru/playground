extern crate rand;
extern crate yaml_rust;

use std::io;
use std::io::Read;
use std::cmp::Ordering;
use rand::Rng;
use std::fs;
use std::path::PathBuf;
use yaml_rust::yaml;

fn main() {
    // load language config
    let lang = translate_from_config("equal", Lang::FR);

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

const TRADUCTION_PATH: &'static str = "res/lang.yaml";

fn translate_from_config(ref_key: &str, lang: Lang) -> String {
    let path = PathBuf::from(TRADUCTION_PATH);

    let path = match fs::canonicalize(&path) {
        Err(_) => panic!("File doesn't exist: {:?}", path),
        Ok(f) => f,
    };

    let mut file = match fs::File::open(&path) {
        Err(_) => panic!("Incorrect path: {:?}", path),
        Ok(f) => f,
    };

    let mut yaml = String::new();
    file.read_to_string(&mut yaml)
        .expect("Unable to parse ressources!");

    let docs = yaml::YamlLoader::load_from_str(&yaml).unwrap();
    match translate(&docs[0], ref_key, &lang) {
        Some(s) => s,
        _ => String::from(ref_key),
    }
}

#[derive(Copy, Clone)]
enum Lang {
    FR,
    EN,
}

impl<'a> Into<Lang> for &'a str {
    fn into(self) -> Lang {
        match self {
            "fr" | "FR" => Lang::FR,
            _ => Lang::EN,
        }
    }
}

impl<'a> Into<String> for &'a Lang {
    fn into(self) -> String {
        match self {
            &Lang::FR => String::from("fr"),
            &Lang::EN => String::from("en"),
        }
    }
}

fn translate(yaml: &yaml::Yaml, ref_key: &str, ref_lang: &Lang) -> Option<String>  {
    let mut key_node = yaml::Yaml::Null;

    match yaml {
        &yaml::Yaml::Hash(ref h) => {
            for (k, v) in h {
                let key: &str = k.as_str().unwrap();
                if key == ref_key {
                    key_node = v.clone();
                }
            }
        },
        _ => {},
    }

    let key_node = match key_node {
        yaml::Yaml::Hash(h) => h,
        _ => panic!("No value was found !"),
    };

    for (l, v) in key_node {
        let value = String::from(v.as_str().unwrap());
        let key_lang = l.as_str().unwrap().to_lowercase();
        let lang: String = ref_lang.into();

        match key_lang.cmp(&lang) {
            Ordering::Equal => {
                return Some(value);
            }
            _ => continue,
        }
    }

    None
}

enum GResult {
    Quit,
    BadParsing,
    Guess(Ordering),
}

fn answer(a: Ordering) {
    match a {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
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
