use increasing::{generic::increasing_generic, Increasing};

mod increasing;

fn main() {
    let v = vec![1, 2, 4, 2, 1, 5, 0];
    let res = increasing::u32::increasing_u32(v.clone().into_iter());
    println!("increasing_u32:");
    for item in res {
        println!("{item}");
    }
    let res = increasing_generic(v.into_iter());
    println!("increasing_generic:");

    for item in res {
        println!("{item}");
    }
    println!("trait Increasing:");
    let strings = vec![String::from("foo"), String::from("bar"), String::from("zoo"), String::from("art")];
    for s in strings.into_iter().increasing(){
        println!("{s}");
    }
}
