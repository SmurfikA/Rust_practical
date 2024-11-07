#[test]
fn test1() {
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);
}

#[test]
fn test2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

#[test]
fn test3() {
    let s = give_ownership();
    println!("{}", s);
}
fn give_ownership() -> String {
    let s = String::from("Hello world");
    let _s = s.as_bytes();
    s
}

#[test]
fn test4() {
    let s = String::from("Hello World");
    print_str(s.clone());
    println!("{}", s);
}
fn print_str(s: String)  {
    println!("{}",s)
}

#[test]
fn test5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

#[test]
fn test6() {
    let s = String::from("Hello ");
    let mut s1 = s;
    s1.push_str("World!");
    println!("Success!");
}

#[test]
fn test7() {
    let x = Box::new(5);
    let mut y = Box::new(3);     // update
    *y = 4;
    assert_eq!(*x, 5);
    println!("Success!");
}

#[test]
fn test8() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{:?}", t.1);
}

#[test]
fn test9() {
    let t = (String::from("hello"), String::from("world"));
    // Fill the blanks
    let (ref s1, ref s2) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}

#[test]
fn test10() {
    let x = 5;
    // Fill the blank
    let p = &x;
    println!("the memory address of x is {:p}", p)
}

#[test]
fn test11() {
    let x = 5;
    let y = &x;
    assert_eq!(&x, y);
    println!("Success!");
}

#[test]
fn test12() {
    let s = String::from("hello, ");
    borrow_object(&s);
    println!("Success!");
}
fn borrow_object(_s: &String) {}

#[test]
fn test13() {
    let mut s = String::from("hello, ");
    push_str(&mut s);
    println!("Success!");
}
fn push_str(s: &mut String) {
    s.push_str("world")
}

#[test]
fn test14() {
    let mut s = String::from("hello, ");
    // Fill the blank to make it work
    let p = &mut s;
    p.push_str("world");
    println!("Success!");
}

#[test]
fn test15() {
    let c = '!';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1),get_addr(r2));
    println!("Success!");
}
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

#[test]
fn test16() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    println!("Success!");
}

#[test]
fn test17() {
    let mut s = String::from("hello, ");
    borrow_object1(&mut s);
    println!("Success!");
}
fn borrow_object1(_s: &mut String) {}

#[test]
fn test18() {
    let mut s = String::from("hello, ");
    borrow_object2(&s);
    s.push_str("world");
    println!("Success!");
}
fn borrow_object2(_s: &String) {}

#[test]
fn test19() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
}

#[test]
fn test20() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}