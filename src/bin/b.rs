#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let mut row = vec![false; h];
    let mut col = vec![false; w];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                row[i] = true;
                col[j] = true;
            }
        }
    }
    for i in 0..h {
        if row[i] {
            let mut print = String::new();
            for j in 0..w {
                if col[j] {
                    print.push(a[i][j]);
                }
            }
            println!("{}", print);
        }
    }
}
