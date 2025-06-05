// loops: loop (unconditional loop-nonstop), while, for-loop
// control flow = conditions & loops
fn loops() {
//     loop{
//         println!("hello world");
//     }
    let mut counter = 0;
    let result = loop{
        counter+=1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("the result is: {result}");
    let mut val = 10;
    let a = &mut val;
    *a += 1;
    println!("{a}");
    println!("{val}");

}

fn simple_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
let mut _a = 0u16;
    let arr = [1, 2, 3, 4, 5];
//     while a < 10 {
//         println!("{a}");
//         a+=1;
//     }

    for mut element in arr {
//         let element = &mut element;
        element+=1;
        println!("{element}");
    }
    for element in arr {
        println!("{element}");
    }
}

fn nested_loop() {
    let mut i = 1;

    loop {
        if i > 3 {
            break;
        }

        let mut j = 1;

        loop {
            if j > 3 {
                break;
            }

            print!("{}x{}={}  ", i, j, i * j);
            j += 1;
        }

        println!(); // New line after inner loop
        i += 1;
    }
}
