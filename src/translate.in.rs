extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
enum Lang {
    FR,
    EN,
}

impl<'a> From<&'a str> for Lang {
    fn from(s: &str) -> Self {
        match s {
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

#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct Translator {
    data: HashMap<String, HashMap<String, String>>,
}

impl Translator {
    fn new(map: HashMap<String, HashMap<String, String>>) -> Translator {
        Translator { data: map }
    }

    fn new_from_config(path_name: &str) -> Translator {
        let mut path = PathBuf::from(path_name);
        path = match fs::canonicalize(&path) {
            Err(_) => panic!("File doesn't exist: {:?}", path),
            Ok(f) => f,
        };

        let mut file = match fs::File::open(&path) {
            Err(_) => panic!("Incorrect path: {:?}", path),
            Ok(f) => f,
        };

        let mut json = String::new();
        file.read_to_string(&mut json)
            .expect("Unable to parse ressources!");

        Translator::new(serde_yaml::from_str(json.as_str()).unwrap())
    }

    fn translate(&self, ref_key: &str, ref_lang: &Lang) -> Option<String>  {
        let pack = match self.data.get(ref_key) {
            Some(h) => h,
            _ => panic!("Key is not found !"),
        };

        for (l, v) in pack {
            let key_lang = l.as_str().to_lowercase();
            let lang: String = ref_lang.into();

            match key_lang.cmp(&lang) {
                Ordering::Equal => {
                    return Some(v.clone());
                }
                _ => continue,
            }
        }

        None
    }
}
