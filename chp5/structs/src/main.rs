fn main() {
    //struct definition
    struct Message {
        hello: String,
    }  

    //Struct instantiation
    let mut my_message = Message {
        hello: String::from("world"),
    };

    println!("Hello, {}!", my_message.hello);

    my_message.hello = String::from("wargle");

    println!("Hello, {}!", my_message.hello);

    //simple returning of new struct
    fn make_message(hello: String) -> Message {
        Message { hello }
    }

    let another_message = make_message(String::from("howdy"));

    println!("Hello, {}!", another_message.hello);



    //user
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user_1 = build_user(String::from("howdy@howdy.com"), String::from("bargle"));

    println!("Hello, {}!", user_1.username);

    let user_2 = User {
        active: user_1.active,
        username: user_1.username,
        email: String::from("another@example.com"),
        sign_in_count: user_1.sign_in_count,
    };

    //Struct update syntax
    //simple types are copied, heap are moved (!)
    let user_3 = User {
        email: String::from("anotherone@example.com"),
        ..user_2
    };

    println!("Hello, {}!", user_3.email);
    println!("Hello, {}!", user_3.username);

    //tuple
    let howdy = (String::from("ho"), String::from("hoholo"));

    println!("Hello, {}!", howdy.0);
    println!("Hello, {}!", howdy.1);

    //tuple structs
    struct Howdy(String, String);

    let new_howdy = Howdy(String::from("bo"), String::from("bobolo"));

    println!("Hello, {}!", new_howdy.1);

    //unit-like structs (similar to () )
    struct BowdyBo;

    let bowdy_bo = BowdyBo;


}
