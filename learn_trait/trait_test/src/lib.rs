pub trait Summary{
    fn summarize(&self) -> String;

    fn default_func(&self) -> String {
        String::from("Default")
    }
}

pub trait Another{
    fn func(&self) {
        println!("Another trait");
    }
}

pub struct News {
    pub title: String,
    pub content: String,
    pub num: i32,
}

pub struct Tweet {
    pub user: String,
    pub content: String,
}

impl Summary for News{
    fn summarize(&self) -> String {
        format!("[{}]{}: {}.", self.num, self.title, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} @ {}", self.content, self.user)
    }
}

impl Another for Tweet {}

impl Another for News {}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
