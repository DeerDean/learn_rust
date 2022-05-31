use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    // raw pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    // deref raw pointer
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // call unsafe function
    unsafe {
        dangerous();
    }

    // create the safe abstraction of unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // let (a, b) = r.split_at_mut(3); // unsafe code inside 
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // extern & foreign function interface
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // visit static mut var
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    
}

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();   // *mut i32

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}