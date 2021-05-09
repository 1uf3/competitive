use proconio::input;

fn main() {
    input!{ mut a: u64, b: u64 }
    let mut i=0;
    while i < b {
        a = if a % 200 == 0 { a / 200 } else { a * 1000 + 200 };
        i+=1;
    }
    println!("{}",a )
    
}
