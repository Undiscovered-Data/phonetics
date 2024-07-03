use std::collections::HashMap;
use std::path::PathBuf;
use rphonetic::{BeiderMorseBuilder, ConfigFiles, Encoder, LanguageSet, RuleType};

use std::fs::read_to_string;

fn main() {
    let allwords = read_lines("./w10_000english.txt");
    let conf_file = ConfigFiles::new(&PathBuf::from("./link_to_beidermorse_rules")).expect("not found");
    let builder = BeiderMorseBuilder::new(&conf_file).rule_type(RuleType::Exact);
    let beider_morse = builder.build();
    let language_set = LanguageSet::from(vec!["english"]);

    let mut count_map: HashMap<String, i32> = HashMap::new();

    for a in allwords {
        let the_code = beider_morse.encode_with_languages(&a, &language_set);
        //let the_code = the_code_s.as_str(); 
        match count_map.get(&the_code) {
            Some(&the_number) => {
                let new_number = the_number + 1;
                count_map.insert(the_code, new_number);
            },
            None => {
                count_map.insert(the_code, 1);
            },
        }
    }
    let hash_len = count_map.len();
    println!("Hash map is {hash_len} long");
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
