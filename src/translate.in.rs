use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
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

#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct Translator {
    data: HashMap<String, HashMap<String, String>>,
}

const TRADUCTION_PATH: &'static str = "res/lang.yaml";

impl Translator {
    fn new(map: HashMap<String, HashMap<String, String>>) -> Translator {
        Translator { data: map }
    }

    fn new_from_config() -> Translator {
        let mut path = PathBuf::from(TRADUCTION_PATH);
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

    fn translate(&self, ref_key: &String, ref_lang: &Lang) -> Option<String>  {
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
