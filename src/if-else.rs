// if-else
fn conditionals(){
    let age: u16 = 18;
    let cond: bool = false;
    if age >= 18 && cond {
        println!("You can get drive a car")
    } else if age >= 16 {
        println!("you can get an ID card")
    } else {
        println!("you are still a kid")
    }

    let res = if cond {"yeah"} else {"no"};
    println!("{res}");
}