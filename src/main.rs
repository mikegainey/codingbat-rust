#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]


fn main() {
    let mut a = [1,2,3,4];
    println!("{:?}", a);

    swap_ends(&mut a);
    println!("{:?}", a);
}

fn swap_ends(a: &mut [i32]) -> &[i32] {
    let len = a.len();
    let swap_var = a[0];
    a[0] = a[len-1];
    a[len-1] = swap_var;
    a
}
