pub trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() -> () {
    let news = NewsArticle {
        headline: String::from("후쿠시마 원전 시찰단 21명 확정…21일부터 5박6일간"),
        location: String::from("한겨레"),
        author: String::from("신민정 기자"),
        content: String::from("정부가 일본 후쿠시마 원전 오염수 현장 시찰단 규모를 21명으로 확정하고 오는 21일부터 5박6일간 파견하기로 했다."),
    };

    println!("{}", news.summarize());

    let tweet = Tweet {
        username: String::from("Nobody"),
        content: String::from("else..."),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
}

