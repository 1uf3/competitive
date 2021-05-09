use proconio::input;

fn main() {
    input!{ 
        n: u32,
        a: [i32; n],
    }

    let mut ans;
            ans = 0;
    for i in 0..n {
        for j in i+1..n {
            if (a[i as usize] - a[j as usize]) % 200 == 0 { 
                ans+=1;
            }else{
                continue;
            }
        }
    }

    println!{"{}", ans}
    
}
