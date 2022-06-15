use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) ->i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}

fn main() {
    // static lifetime
    let s: &'static str = "123";
    // equal to -> let s = "123";

    let s1 = String::from("12345");
    let long;

    {
        let s2 = String::from("123");
        long = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", long);
    }
    
    // Alougth s1 is longer,
    // "s2.as_str()" borrowed value does not live long enough
    // println!("Longest: {}", long);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);

    i.announce_and_return_part("announcement");

    let ss1 = String::from("12345");
    let ss2 = String::from("123");
    longest_with_an_anouncement(ss1.as_str(),ss2.as_str(),"announcement");
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_anouncement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// "result.as_str()" is a dangling refference
// fn longest1<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }
