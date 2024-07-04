use std::collections::HashMap;
use std::{
    fs::read_to_string,
    io::{self, BufRead, BufReader},
    path::Path,
};

use rphonetic::{Caverphone1, Caverphone2, Encoder};

fn main() {

    let allwords = read_lines("./w10_000english.txt");
    let allwords2 = allwords.clone();
    let caverphone1 = Caverphone1;
    let caverphone2 = Caverphone2;
/*
    let john1 = caverphone1.encode("Johnson");
    let john2 = caverphone2.encode("Johnson");
    println!("{:?}", john1);
    println!("{:?}", john2);
*/
    let mut count_map: HashMap<String, i32> = HashMap::new();
    for a in allwords {
        let the_code = caverphone1.encode(&a);
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

    let mut count_map2: HashMap<String, i32> = HashMap::new();
    for a in allwords2 {
        let the_code = caverphone2.encode(&a);
        match count_map2.get(&the_code) {
            Some(&the_number) => {
                let new_number = the_number + 1;
                count_map2.insert(the_code, new_number);
            },
            None => {
                count_map2.insert(the_code, 1);
            },
        }
    }
    let hash_len = count_map2.len();
    println!("Hash map is {hash_len} long");

}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
