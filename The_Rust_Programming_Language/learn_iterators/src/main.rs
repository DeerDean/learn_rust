fn main() {
    println!("Hello, world!");
}

// create a iterator
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    // the iter must be mutable! 'next()' will change the iter state
    let mut v1_iter = v1.iter();    // get refenrence
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);

    // get mutbale reference
    // the vec must be mutable also
    let mut v2 = vec![String::from("a"), String::from("b"), String::from("c")];
    let mut v2_iter = v2.iter_mut();    
    let mut a = String::from("a");
    assert_eq!(v2_iter.next(), Some(&mut a));

    let v3 = vec![String::from("a"), String::from("b"), String::from("c")];
    let mut v3_iter = v3.into_iter();    // get ownership
    let a = String::from("a");
    assert_eq!(v3_iter.next(), Some(a));
}


// consuming adaptors
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // v1_iter moved!
    assert_eq!(total, 6);
    // let total2: i32 = v1_iter.sum(); 
}

// iterator adaptors
#[test]
fn iterator_adaptors() {
    let v1: Vec<u32> = vec![1, 2, 3];

    // iter is lazy, need collect() to consume
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

// use closure to get env args -----------------------
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoe_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

// Iterator trait ----------------------------
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_dirctly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods(){
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a*b)
        .filter(|x| x % 3 ==0)
        .sum();
    assert_eq!(18, sum);


    let sum: u32 = Counter::new().map(|x| x + 1).sum();
    assert_eq!(20, sum);

    let mut itr = Counter::new().map(|x| x+ 1);
    let mut c = 1;
    for n in itr {
        c += 1;
        assert_eq!(c ,n);
    }
}
