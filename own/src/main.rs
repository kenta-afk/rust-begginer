#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    {
        let s = "hello";
        println!("{}", s);
    }
    
    //prnitln!("{}", s);
    {
        let mut s: String = String::from("hello");

        s.push_str(", world!");

        println!("{}", s);
    }
    // OSにメモリを返す 


    let user1: User = build_user("hoge@gmail.com".to_string(), "hoge".to_string());

    println!("{:?}", &user1);

    let user2: User = User {
        email: String::from("hogehoge@gmail.com"),
        name: String::from("hogehoge"),
        ..user1
    };

    println!("{:?}", &user2);
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}
