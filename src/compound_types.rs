// compound data types: arrays, tuples, slices, and strings (slice string)

fn main(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("number i32 array: {:?}", numbers);
    // &str -> reference to a string, string slice - immutable
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("fruits array: {:?}", fruits);
    println!("fruits first element: {}", fruits[0]);
    println!("fruits second element: {}", fruits[1]);
    println!("fruits third element: {}", fruits[2]);

    // tuples
    let _human: (&str, i32, bool) = ("Alice", 30, true);
    let _human_2: (String, i32, bool) = ("Alice".to_string(), 30, true);
    let my_mix = ("Kratos", 23, false, [1, 2, 3]);
    println!("my mix tuple: {:?}", my_mix);

    // slices: [1, 2, 3, 4, 5]
    let num_slices: &[i32] = &[1, 2, 3, 4, 5];
    let animal_slices: &[&str] = &["IT", "elephant"];
    let books_slices: &[&String] = &[&"IT".to_string(), &"elephant".to_string()];
    println!("animal slices: {:?}", animal_slices);
    println!("num slices: {:?}", num_slices);
    println!("books_slices slices: {:?}", books_slices);

    //strings -> growable, mutable, owned string types
    //strings vs string slices (&str)
    let mut stone_cold: String = String::from("Hello, ");
    stone_cold.push_str("Yeah");
    println!("stone says: {}", stone_cold);

    //B- &str (String Slice)
    let hello: String = String::from("Hello, World!");
    let slice: &str = &hello;
    let _slice2: &str = &hello[0..5];
    println!("slice value: {}", slice);
}