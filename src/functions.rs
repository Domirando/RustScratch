// Functions
// main - entry point
// use only snake case: hello_world
fn functions() {
    hello_world();
    tell_height(32);
    human_id("maftuna", 18, 162.5);
    //expression
    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("result is {}", x);
    println!("{}", sum(4, 6));
}

// const _X = {
//     //code
// };

fn hello_world(){
    println!("hello world")
}

fn tell_height(height: i32){
    println!("my height is: {}cm.", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("my name is {}, i am {} years old, my height is {}", name, age, height)
}

// expressions (return) and statements(anything not returning)
fn sum(a: i32, b: i32) -> i32 {
    a+b
}

//{:.2} -> means display only two decimal points that's after the point like 6.56 or 15.14