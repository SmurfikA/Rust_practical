#[test]
fn test1() {
    println!("Success!");
}
#[test]
fn test() {
    let s: &str = "hello, world";
    greetings(s)
}
fn greetings(s: &str) {
    println!("{}",s)
}
#[test]
fn test3() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
    println!("{}", s);
    println!("Success!");
}

#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

#[test]
fn test5() {
    let mut s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test6() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str();
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

#[test]
fn test7() {
    let s: &str = "hello, world";
    greetings(String::from(s))
}

fn greetings(s: String) {
    println!("{}", s)
}

#[test]
fn test8() {
    let s: String = "hello, world".to_string();
    let s1: &str = s.as_str();

    println!("Success!");
}

#[test]
fn test9() {
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test10() {
    let s1: String = String::from("hi,world");
    let h: &str = &s1[0..1];
    assert_eq!(h, "h");

    let h1 = &s1[3..6];
    assert_eq!(h1, "!");

    println!("{}", h1);

    println!("Success!");
}

#[test]
fn test11() {
    for c in "kk，dd".chars() {
        println!("{}", c)
    }
}

#[test]
fn test12() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);
    println!("Success!");
}

#[test]
fn test13() {
    let _arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    assert_eq!(size_of_val(&arr), 12);
    println!("Success!");
}

#[test]
fn test14() {
    let list: [i32; 100] = [1; 100] ;
    assert_eq!(list[0], 1);
    assert_eq!(list.len(), 100);
    println!("Success!");
}

#[test]
fn test15() {
    let _arr: [i32; 3] = [1, 2, 3];
    println!("Success!");
}

#[test]
fn test16() {
    let arr = ['a', 'b', 'c'];
    let ele = arr[0];
    assert_eq!(ele, 'a');
    println!("Success!");
}

#[test]
fn test17() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let _name0 = names.get(0).unwrap();
    let _name1 = &names[1];
    println!("Success!");
}

#[test]
fn test18() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];
    let _s2: &str = &("hello, world" as &str);
    println!("Success!");
}

#[test]
fn test19() {
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    assert_eq!(size_of_val(&slice), 16);
    println!("Success!");
}

#[test]
fn test20() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("Success!");
}

#[test]
fn test21() {
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);
    println!("Success!");
}

#[test]
fn test22() {
    let s = "你好，世界";
    let slice = &s[0..3];
    assert_eq!(slice, "你");
    println!("Success!");
}

#[test]
// Fix errors
fn test23() {
    let mut s = String::from("hello world");
    let letter = first_letter(&s);
    println!("the first letter is: {}", letter);
    s.clear(); // error!
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}

#[test]
fn test24() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("Success!");
}

#[test]
fn test25() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
    println!("Success!");
}

#[test]
fn test26() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12); //13);
    println!("too long tuple: {:?}", too_long_tuple);
}

#[test]
fn test27() {
    let tup = (1, 6.4, "hello");
    // Fill the blank to make the code work
    let (x, z, y) = tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
    println!("Success!");
}

#[test]
fn test28() {
    //Fill the blank
    let (a, b, c) = (1, 2, 3);
    let (x, y, z) = (c, a, b);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
    println!("Success!");
}

#[test]
fn test29() {
    let (x, y) = sum_multiply((3, 2));
    assert_eq!(x, 5);
    assert_eq!(y, 6);
    println!("Success!");
}
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

#[test]
fn test30() {
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("reading"),
    };
    println!("Success!");
    struct Person {
        name: String,
        age: u8,
        hobby: String
    }
}

#[test]
fn test31() {
    struct Unit;
    trait SomeTrait {
    }

    impl SomeTrait for Unit {  }
    let u = Unit;
    do_something_with_unit(u);
    println!("Success!"); //на раст бай практіс не виводить
    fn do_something_with_unit(_u: Unit) {   }
}

#[test]
fn test32() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let v = Point(0, 127, 255);
    check_color(v);
    println!("Success!");
    fn check_color(p: Point) {
        let Point(x, _, y) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(y, 255);
    }
}

#[test]
fn test33() {
    struct Person {
        name: String,
        age: u8,
    }
    let age = 18;
    let mut p = Person {
        name: String::from("facebook"),
        age,
    };
    p.age = 30;
    // Fill the blank
    p.name = String::from("fei");
    println!("Success!");
}

#[test]
fn test34() {
    struct Person {
        name: String,
        age: u8,
    }
    println!("Success!");
    fn build_person(name: String, age: u8) -> Person {
        Person {
            age,
            name,
        }
    }
}

#[test]
fn test35() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let u1 = User {
        email: String::from("face@example.com"),
        username: String::from("facebook"),
        active: true,
        sign_in_count: 1,
    };
    let _u2 = set_email(u1);
    println!("Success!");
    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}

#[test]
fn test36() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    println!("{:?}", rect1);
}

#[test]
fn test37() {
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };
    let _name = f.name;
    println!("{}, {}", _name, f.data);
}

#[test]
fn test38() {
    enum Number {
        Zero,
        One,
        Two,
    }
    enum Number1 {
        Zero = 0,
        One,
        Two,
    }
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
    println!("Success!");
}

#[test]
fn test39() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let _msg1 = Message::Move{x: 1, y: 2};
    let _msg2 = Message::Write(String::from("hello, world!"));
    println!("Success!");
}

#[test]
fn test40() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::Move{x: 2, y: 2};
    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
    println!("Success!");
}

#[test]
fn test41() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];
    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        println!("{:?}", msg);
    }
}

#[test]
fn test42() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);
    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    } else {
        panic!("NEVER LET THIS RUN！");
    }
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}