#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
extern crate regex;

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
    let mut transCount : TransferCount = TransferCount::new();
    let mut resultCount : HashMap<String, i32> = HashMap::new();
    for tweet in tweets.iter() {
        // Clean
        lazy_static! {
            static ref RE_NONALPHA : regex::Regex = regex::Regex::new(r"[^A-Za-z\s@#_\-,!?.]").unwrap();
            static ref RE_URL : regex::Regex = regex::Regex::new(r"https?://[^\s]+").unwrap(); // This is very greedy!
            static ref RE_TERMINATORS : regex::Regex = regex::Regex::new(r"([^A-Z])([!.?]+)(\s|$)").unwrap();
        }
        let clean_tweet : String = RE_URL.replace_all(tweet, "").trim().into();
        let clean_tweet : String = RE_TERMINATORS.replace_all(&clean_tweet, "$1$2 __END__SPLIT__START ").into();
        let clean_tweet : String = RE_NONALPHA.replace_all(&clean_tweet, "").replace("\n","").trim().into();
        let clean_tweet = format!("{} {} {}","__START", clean_tweet, "__END");
        for s in clean_tweet.split("__SPLIT")
        {
            if s != "__START __END" {
                sentences.push(s.into());
            }
        }
        // Tokenise
    }
    for (i,s) in sentences.iter().enumerate() {
        let mut prevWord : String = String::from("");
        for w in s.split(" ")
        {
            if i > 0 {
                let key : (Key, String) = (prevWord,w.into());
                *transCount.entry(key).or_insert(0) += 1; // update transfer count
                *resultCount.entry(w.into()).or_insert(0) += 1; //update normalisation
            }
            prevWord = w.into();
        }
    }
    for (key, count) in transCount {
        let totalCount = resultCount.get(&key.1).unwrap();
        // println!("{},{} -> {} of {}", key.0, key.1, count, totalCount);
        probs.insert(key, (count as f32) / (*totalCount as f32));
    }
    for (key, p) in probs {
        println!("{},{} -> {}", key.0, key.1, p);
    }
}

fn generate(probs: &TransferMatrix) -> String {
    let mut tweet =  String::new();
    tweet.push_str("This is an auto generated tweet. Sad!");
    return tweet;
}
