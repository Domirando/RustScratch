// references : mutable (can be modified) and immutable (default/ cant be changed)
fn references() {
    let mut a = 5;
    let b = &mut a;
    *b += 1;
    a+=3;
    // while printing borrowed ones we can't print both mutable (Can be changed) and immutable at the same time, we have to print only one mutable or several immutable (cannot be changed) at the same time or do them separately
    println!("mutable b is equal to {}", b);
    println!("mutable a is equal to {}", a);
}