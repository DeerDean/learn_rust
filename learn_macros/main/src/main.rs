use mac;
use mac::HelloMacro;
use mac_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn main() {
    let v = mac::my_vec![1,2,3];
    println!("{:?}", v);

    Pancakes::hello_macro();
}
