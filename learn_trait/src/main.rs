use trait_test::{Another, News, Summary, Tweet};

fn main() {
    let mut t = Tweet {
        user: String::from("Bob"),
        content: String::from("Hello!"),
    };

    let s = t.summarize();
    println!("{}", s);

    println!("{}", t.default_func());

    my_print(&t);

    my_print2(&t);
}

// params has traits 
fn my_print<T: Summary + Another>(item: &T) {
    println!("print:");
    println!("{}", item.summarize());
    item.func();
}

// use "where"
fn my_print1<T>(item: &T)
where
    T: Summary + Another,
{
    println!("print:");
    println!("{}", item.summarize());
    item.func();
}

// another way
fn my_print2(item: &(impl Summary + Another)) {
    println!("print2:");
    println!("{}", item.summarize());
}
