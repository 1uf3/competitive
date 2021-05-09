use proconio::input;

fn main() {

    input!{ a: u32 }
    println!("{}", if a > a / 100 * 100 { a / 100 + 1 } else { a / 100 })
    
}
