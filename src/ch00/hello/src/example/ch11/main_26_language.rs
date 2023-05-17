#[derive(Debug)]
enum Language {
    Spanish,
    German,
    Italia,
    English,
    Korean,
    Japanese,
}


fn main() {

    let es = Language::Spanish;
    let de = Language::German;
    let it = Language::Italia;
    let en = Language::English;
    let ko = Language::Korean;
    let ja = Language::Japanese;

    hello(&es);
    hello(&es);
}

fn hello(lang: &Language) {
    println!("Hello, {:?}!", lang);
}
