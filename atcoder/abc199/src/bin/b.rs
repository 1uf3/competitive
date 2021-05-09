use proconio::input;


fn main() {
    input! { n: usize, a: [i32; n], b: [i32; n] }
    let ans = (b.iter().min().unwrap() - a.iter().max().unwrap() + 1).max(0);
    println!("{}", ans);
}
