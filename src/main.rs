fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 {
        None
    } else {
        Some(x / y)
    };
    ans
}

fn add_i32(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test1(){
    assert_eq!(add_i32(10, 23), 33)
}


fn main() {
    println!("Hello, world!");
    println!("{:?}",func_ex_div_some(20, 10));
    println!("{}", add_i32(22, 11));
}
