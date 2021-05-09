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

/* Answer
 *
 * fn main() {
 *   input!{
 *     n: u32,
 *     a: [i32, n],
 *   }
 *   let mut b: [i64, 200];
 *   for i in 0..n {
 *     b[a[i]%200]+=1;
 *   }
 *   let mut ans: i64;
 *   for i in 0..200 {
 *     ans+=(b[i]*(b[i]-1)/2)
 *   }
 *   println!("{}", ans)
 *
 * }
 *
 */
