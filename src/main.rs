#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
extern crate regex;
extern crate rand;

type Key = String;
type TransferCount = HashMap< (Key, String) , i32>;
type TransferMatrix = HashMap< (Key, String) , f32>;
type TweetData = Vec<String>;


fn main() {
    let mut file = File::open("tweets.json").expect("Couldnt open file");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let parsed : Vec<HashMap<String, String>> = serde_json::from_str(&data).unwrap();

    let mut all_tweets : TweetData = Vec::new();
    for elem in parsed.iter() {
        all_tweets.push(elem["text"].clone());
    }

    let mut transfer_probs = TransferMatrix::new();
    learn(&mut transfer_probs, &all_tweets);

    println!("{}", generate(&transfer_probs));

}

fn learn(probs: &mut TransferMatrix, tweets: &TweetData) {
    let mut sentences : Vec<String> = Vec::new();
    let mut trans_count : TransferCount = TransferCount::new();
    let mut result_count : HashMap<Key, i32> = HashMap::new();
    for tweet in tweets.iter() {
        // Clean
        lazy_static! {
            static ref RE_NONALPHA : regex::Regex = regex::Regex::new(r"[^A-Za-z\s@#_\-,!?.0-9]").unwrap();
            static ref RE_URL : regex::Regex = regex::Regex::new(r"https?://[^\s]+").unwrap(); // This is very greedy!
            static ref RE_TERMINATORS : regex::Regex = regex::Regex::new(r"([^A-Z])([!.?]+)(\s|$)").unwrap();
        }
        let clean_tweet : String = RE_URL.replace_all(tweet, "").trim().into();
        let clean_tweet : String = RE_TERMINATORS.replace_all(&clean_tweet, "$1$2 __SPLIT").into();
        let clean_tweet : String = RE_NONALPHA.replace_all(&clean_tweet, "").replace("\n","").trim().into();

        for s in clean_tweet.split("__SPLIT")
        {
            if s != "" {
                let s = format!("{} {} {}","__START1", s, "__END");
                sentences.push(s.into());
            }
        }
        // Tokenise
    }

    // println!("{:?}", sentences);
    for s in sentences {
        // let mut prev_word1 : String = String::from("");
        let mut prev_word2 : String = String::from("");
        for (i,w) in s.split(" ").enumerate()
        {
            if !w.trim().eq("") {
                if i>0{
                    let key : Key = prev_word2.clone();
                    let item : (Key, String) = (prev_word2.into(),w.trim().into());
                    *trans_count.entry(item).or_insert(0) += 1; // update transfer count
                    *result_count.entry(key).or_insert(0) += 1; //update normalisation
                }
                // prev_word1 = prev_word2.clone();
                prev_word2 = w.trim().into();
            }
        }
    }
    for (key, count) in trans_count {
        let total_count = result_count.get(&key.0).unwrap();
        // println!("{},{} -> {} of {}", key.0, key.1, count, total_count);
        probs.insert(key, (count as f32) / (*total_count as f32));
    }
    // for (key, p) in probs {
    //     println!("{:?},{} -> {}", key.0, key.1, p);
    // }
}

fn generate(probs: &TransferMatrix) -> String {
    let mut tweet =  String::new();
    let mut words : Vec<String> = Vec::new();
    words.push(String::from("__START1"));
    // words.push(String::from("__START2"));
    let mut sentence_done = false;
    while !sentence_done {
        // let val : f32;
        let mut word_done = false;
        let mut prob_sum = 0.0;
        let rand::Open01(rand_tgt) = rand::random::<rand::Open01<f32>>();
        let mut it = probs.iter();
        while !word_done {
            let (item, p) = match it.next() {
               Some(x) => x,
               None => break,
            };
            let w = &item.1;

            if item.0.eq(words.last().unwrap()) {
                // println!("{:?}=={:?}? -> {} p={} sum={} tgt={}", words.last().unwrap(), item.0, w, p, prob_sum, rand_tgt);
                if prob_sum + p > rand_tgt {
                    word_done = true;
                    words.push(w.clone());
                }
                prob_sum += *p;
            }
        }

        if words.last().unwrap().eq("__END") {
            let char_count = words
                .iter()
                .map(|name: &String | name.len())
                .fold(0, |acc, len| acc + len );
            // println!("{} -> {:?}", char_count, words);
            if char_count > 140 {
                sentence_done = true;
            }
            else {
                words.push(String::from("__START1"));
            }
            // sentence_done = true;
        }

    }
    // println!("{:?}", words);

    tweet.push_str(words.join(" ").replace("__START1 ","").replace("__END","").trim());
    return tweet;
}
