//constants - always with capital letters + define data type + can be declared in global scope + can't be converted to mutable

fn constants(){
    println!("this is A: {}", A);
    const B: &str = "hello";
    println!("this is B: {}", B);
}

const A: f32 = 15.36;