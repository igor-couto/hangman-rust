extern crate rand;

use std::{fs::File, io::Read};

use rand::Rng;

pub fn pick_a_word() -> String {
    let mut file = File::open("words.txt")
        .unwrap_or_else(|_| panic!("Arquivo com as palavras não foi encontrado: words.txt"));

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)
        .expect("Falha ao ler arquivo.");

    let words: Vec<&str> = file_content.as_str().split_whitespace().collect();

    words[rand::thread_rng().gen_range(0..words.len())].to_owned()
}
