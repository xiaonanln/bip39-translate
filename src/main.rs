use std::env::args;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let mut args = args();

    let eng_wordlist = load_wordlist("wordlists/english.txt");
    let chs_wordlist = load_wordlist("wordlists/chinese_simplified.txt");

    args.next();

    let mut num = 0;
    for word in args {
        let word = word.trim();
        let word_trans;

        let idx = eng_wordlist.find(&word);
        if let Some(idx) = idx {
            word_trans = chs_wordlist.get(idx);
        } else {
            let idx = chs_wordlist.find(&word).expect(&std::format!("Not a valid BIP39 word: {}", word));
            word_trans = eng_wordlist.get(idx);
        }

        num += 1;
        if num % 4 != 1 {
            print!(" ")
        }

        print!("{}", word_trans);

        if num % 4 == 0 {
            println!("")
        }
    }

    if num % 4 != 0 {
        println!("")
    }
}

const WORDLIST_SIZE: usize = 2048;

struct WordList {
    words: Vec<String>,
}


impl WordList {
    pub(crate) fn len(&self) -> usize {
        self.words.len()
    }
    pub(crate) fn add(&mut self, word: String) {
        assert!(self.len() < WORDLIST_SIZE);
        assert_eq!(word, word.trim());
        self.words.push(word);
    }
    pub(crate) fn find(&self, word: &str) -> Option<usize> {
        assert_eq!(self.len(), WORDLIST_SIZE);
        let word = word.to_lowercase();
        for i in 0..WORDLIST_SIZE {
            if self.words[i] == word {
                return Some(i);
            }
        }
        None
    }
    pub(crate) fn get(&self, idx: usize) -> &String {
        assert!(idx < WORDLIST_SIZE);
        &self.words[idx]
    }
}

fn load_wordlist(path: &str) -> WordList {
    let mut wordlist = WordList { words: vec![] };

    let fin = File::open(path).unwrap();
    let lines = io::BufReader::new(fin).lines();
    for line in lines {
        let line = line.unwrap();
        wordlist.add(line);
    }


    assert_eq!(wordlist.len(), WORDLIST_SIZE);
    wordlist
}
