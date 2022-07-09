fn list_sum(list: &[u32]) -> Option<u32> {
    let mut sum = 0;
    for e in list {
        if sum > (u32::MAX - e) {
            return None;
        }
        sum += e;
    }
    Some(sum)
}

fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    let mut sum = list_sum(&list);
    match sum {
        Some(u) => println!("The sum of list {:?} is: {}", list, u),
        None => println!("Overflow!"),
    }

    list = vec![1, 2, 3, 4, u32::MAX];
    sum = list_sum(&list);
    match sum {
        Some(u) => println!("The sum of list {:?} is: {}.", list, u),
        None => println!("The sum of list {:?} overflows!", list),
    }
    
}
