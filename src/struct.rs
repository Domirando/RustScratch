fn struct_f() {
    //     tuple
    let _rect = (200, 500);

    // a struct (short for "structure") is a custom data type that allows you to group together related data into a single unit.
    struct _Book {
        title: String,
        author: String,
        pages: String,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("domirando"),
        email: String::from("smth@gmail.com"),
        sign_in: 1,
    };

//     user1.email = String::from("sss");
    println!("{}", user1.email);

    fn build_user(email:String, username:String) -> User{
        User {
            active: true,
            email,
            username,
            sign_in: 2,
        }
    }
//     let user2 = User {
//         email: String::from("another@"),
//         ..user1
//     };
    let user2 = build_user(String::from("another@"), String::from("userr"));
    println!("{}", user2.sign_in);
    println!("{}", user2.email);
    println!("{}", user2.username);

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
}