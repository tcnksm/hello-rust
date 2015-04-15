fn main() {
    let mut v = vec![];

    v.push("Hello");

    // This is failed.
    // When we make a reference to v, we let that variable (in this case, x)
    // borrow it for a while. 
    let x = &v[0];
    // let x = v[0].clone();

    v.push("World");

    println!("{}", x);
    println!("{:?}", v);
}
