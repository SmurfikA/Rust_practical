#[test]
fn test1() {
    let x: i32 = 5;
    let mut _y = 5;
    _y = x;
    let _z: i32 = 10; // Type of z ?
    println!("Success!");
}

#[test]
fn test2() {
    let _v: u16 = 38_u8 as u16;
    println!("Success!");
}

#[test]
fn test3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("Success!");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}

#[test]
fn test5() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}

#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert_eq!(v, 1597);
    println!("Success!");
}

#[test]
fn test7() {
    let x = 1_000.000_1;
    let _y: f32 = 0.12;
    let _z = 0.01_f64;
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
fn type_of1<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test8() {
    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32); //or as f32
    println!("Success!");
}

#[test]
fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert_eq!(sum, -5);
    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
use std::ops::{Range, RangeInclusive};
#[test]
fn test10() {
    assert_eq!(1..5, Range{ start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5)); //
    println!("Success!");
}

#[test]
fn test11() {
    // Integer addition
    assert_eq!(1u32 + 2u32, 3u32);

    assert_eq!(1i32 - 2i32, -1i32);
    assert_eq!(1i8 - 2i8, -1i8); // з 1u8 в і8

    assert_eq!(3 * 50, 150);

    assert_eq!(9.6_f32 / 3.2_f32, 3.0_f32);

    assert_eq!(24 % 5, 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert_eq!(!true, false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

#[test]
fn test12() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); // 4байта
    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);
    println!("Success!");
}

#[test]
fn test13() {
    let c1 = '中'; //" " в ' '
    print_char(c1);
}
fn print_char(c : char) {
    println!("{}", c);
}

#[test]
fn test14() {
    let _f: bool = false;
    let t = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test15() {
    let f = false; //було тру
    let t = false; //було let t = true && false;
    assert_eq!(t, f);
    println!("Success!");
}

#[test]
fn test16() {
    let _v: () = ();
    let _s = (2, 3); //not _v? _v:(i32, i32) помилка
    assert_eq!(_v, implicitly_ret_unit());
    println!("Success!");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
}

#[test]
fn test17() {
    let unit: () = ();
    assert_eq!(size_of_val(&unit), 0);
    println!("Success!");
}

#[test]
fn test18() {
    let v = { //or v: i32
        let mut x = 1;
        x += 2 //;
        //x
    };
    assert_eq!(v, ()); //assert_eq!(v, 3);
    println!("Success!");
}

#[test]
fn test19() {
    let v = {
        let x = 3;
        x
    };
    assert_eq!(v, 3);
    println!("Success!");
}

#[test]
fn test20() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y //not ";"
}

#[test]
fn test21() {
    let (x, y) = (1, 2);
    let s = sum1(x, y);
    assert_eq!(s, 3);
    println!("Success!");
}
fn sum1(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test22() {
    print();
}
fn print() -> () {
    println!("Success!")
}

#[test]
fn test23() {
    never_return();
    println!("Failed!");
}
fn never_return() -> () { //було так fn never_return() -> !

}

#[test]
fn test24() {
    println!("Success!");
}
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    never_return_fn()
}
fn never_return_fn() -> ! {
    todo!()
}

#[test]
fn test25() {
    let b = false;
    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for false, but we can panic");
        }
    };
    println!("Exercise Failed if printing out this line!");
}