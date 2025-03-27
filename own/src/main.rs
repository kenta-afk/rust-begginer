#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Kensi {
    name: String,
    katana: String,
    age: u64,
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
    let kensi1 :Kensi = build_kensi("rengoku".to_string(), "honou".to_string(), 20);

    println!("{:?}", &user1);
    println!("{:?}", &kensi1);

    let user2: User = User {
        email: String::from("hogehoge@gmail.com"),
        name: String::from("hogehoge"),
        ..user1
    };

    let kensi2: Kensi = Kensi {
        name: String::from("tokito"),
        katana: String::from("murasame"),
        age: 14,
    };

    println!("{:?}", &user2);
    println!("{:?}", &kensi2);
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

fn build_kensi (name:  String, katana: String, age: u64) -> Kensi {
    Kensi {
        name,
        katana,
        age,
    }
}
