// ownership - out of scope the variable/owner drops its value
// so we can't have multiple owners/variables for the same value
// that's why if we want another variable to have the same value as other one then we have to give a reference to that variable with & symbol in front of the variable name we are referring to

fn ownership() {
    let s1 = String::from("RUST");
    let s2 = s1;
    let length = calculate_length(&s2);
//     println!(" length of {}", s1) doesn't work because s1 lost it's owndership to s2 so it' doesnt have any value anymore
    println!("length of {}, is {}", s2, length)
}
fn calculate_length(s: &String) -> usize {
    s.len()
}