pub fn summarize() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        summarize();
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify<T: Summary>(item: &T) = トレイト境界(trait bound)構文
pub fn notify(item: &impl Summary) {    // <- トレイト境界構文の糖衣構文(syntax sugar)
    println!("Braeking news! {}", item.summarize());
}

// item1 と item2 は Summaryトレイト を実装していればOK
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {}
// item1 と item2 は Summaryトレイト を実装している同じ型でないとダメ（そうしたい場合は糖衣構文が使えない）
pub fn notify3<T: Summary>(item1: &T, item2: &T) {}

use std::fmt::Display;

// 複数のトレイト境界を指定する
pub fn notify4(item: &(impl Summary + Display)) {}
// = notify4<T: Summary + Display>(item: &T) {}

use std::fmt::Debug;
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
// where句を使ってトレイト境界を指定する
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
    0
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/* 異なる型は返せない error[E0308]: `if` and `else` have incompatible types
fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/
