extern crate rand;
extern crate reqwest;
extern crate url;
use rand::prelude::*;
use reqwest::header::*;
use std::thread;
use url::form_urlencoded::byte_serialize;

fn main() {
    println!("Hello, world!");
    let (firstn, lastn, uasn) = read_arrays();
    let mut c: config = config::new("xx".to_owned());
    c.first = firstn;
    c.last = lastn;
    c.uas = uasn;

    spawn(&c);
}

fn read_arrays() -> (Vec<String>, Vec<String>, Vec<String>) {
    let firstf = "first_names.txt";
    let first_names = read_f(firstf);
    let lastf = "last_names.txt";
    let last_names = read_f(lastf);
    let uasf = "user_agents.txt";
    let uasn = read_f(uasf);

    (first_names, last_names, uasn)
}

fn read_f(path: &str) -> Vec<String> {
    let a = std::fs::read_to_string(&path).expect("ayy");
    let b = a.split("\r\n");
    let c: Vec<String> = b.map(|r| r.trim().to_owned()).collect();
    c
}

fn random_from_list(rng: &mut rand::prelude::ThreadRng, lst: &Vec<String>) -> String {
    let s = rand::seq::IteratorRandom::choose(lst.iter(), rng).unwrap();
    s.to_owned()
}

fn request(c: &config, tname: &str) {
    let mut rng = thread_rng();
    for i in 0..c.send_amount {
        println!("Done {} times from thread {}", i, tname);
        let user_agent = random_from_list(&mut rng, &c.uas);
        let name = random_name(&mut rng, &c);
        let name_urlencoded: String = byte_serialize(name.as_bytes()).collect();
        let pollname_urlencoded: String = byte_serialize(&c.pollname.as_bytes()).collect();
        let words: String = if &c.word_override.len() > &0 {
            let cw = c.word_override.to_owned();
            let s: Vec<String> = cw.into_iter().map(|x| x.to_string()).collect();
            let s = s.join(",");
            byte_serialize(s.as_bytes()).collect()
        } else {
            let cw = random_words(&mut rng, c);
            let s: Vec<String> = cw.into_iter().map(|x| x.to_string()).collect();
            let s = s.join(",");
            byte_serialize(s.as_bytes()).collect()
        };
        let dnt = if rng.next_u32() % 2 == 0 { "0" } else { "1" };
        let upgrade = if rng.next_u32() % 2 == 0 { "0" } else { "1" };

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, user_agent.parse().unwrap());
        headers.insert(
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headers.insert(DNT, dnt.parse().unwrap());
        headers.insert(CONNECTION, "keep-alive".parse().unwrap());
        headers.insert(UPGRADE_INSECURE_REQUESTS, upgrade.parse().unwrap());

        let res = reqwest::Client::new()
            .post("https://kevan.org/johari.cgi")
            .headers(headers)
            .body(format!(
                "voter={}&name={}&words={}",
                name_urlencoded, pollname_urlencoded, words
            ))
            .send();
        if let Err(m) = res {
            eprintln!(
                "Thread received an error, shutting this thread down:\n{}",
                m
            );
            break;
        }
    }
}

fn random_words(rng: &mut ThreadRng, c: &config) -> Vec<u32> {
    // how many words will we choose
    let r = rng.next_u32() as usize;
    let x: Vec<u32> = (c.word_choice_min..c.word_choice_max).collect();
    let count = x[r % x.len()];
    // producing a list that will not be replaced
    let mut ids: Vec<u32> = (c.word_id_min..c.word_id_max).collect();

    let mut words: Vec<u32> = Vec::new();
    for _ in 0..count {
        let r2 = rng.next_u32() as usize;
        let rand_idx = r2 % ids.len();
        let word = ids.remove(rand_idx);
        words.push(word);
    }

    words
}

fn random_name(rng: &mut ThreadRng, c: &config) -> String {
    let first = random_from_list(rng, &c.first);
    let mut last = String::new();

    let r = (rng.next_u32() % 100) as f32;
    if r < (c.full_name_percent * 100.0) {
        last = format!(" {}", random_from_list(rng, &c.last));
    }

    let name = format!("{}{}", first, last);
    name
}

#[derive(Clone)]
struct config {
    pollname: String,
    thread_count: usize,
    first: Vec<String>,
    last: Vec<String>,
    uas: Vec<String>,
    word_id_min: u32,
    word_id_max: u32,
    word_choice_min: u32,
    word_choice_max: u32,
    boy_percent: f32,
    full_name_percent: f32,
    middle_initial_percent: f32,
    fake_name_percent: f32,
    send_amount: u32,
    word_override: Vec<u32>,
}

impl config {
    fn new(pollname: String) -> config {
        config {
            pollname: pollname,
            thread_count: 2,
            word_id_min: 1,
            word_id_max: 55,
            word_choice_min: 5,
            word_choice_max: 6,
            boy_percent: 0.5,
            full_name_percent: 0.66,
            middle_initial_percent: 0.1,
            fake_name_percent: 0.99,
            send_amount: 10_000,
            word_override: Vec::new(),
            first: Vec::new(),
            last: Vec::new(),
            uas: Vec::new(),
        }
    }
}

fn spawn(c: &config) {
    let mut threads: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for i in 0..c.thread_count {
        let m = c.clone();
        let t = thread::spawn(move || {
            request(&m, format!("p{}", i).as_ref());
        });
        threads.push(t);
    }

    for t in threads {
        t.join();
    }
}

// fn random_name
