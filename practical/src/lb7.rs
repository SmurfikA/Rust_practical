#[test]
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }
        count += 5;
        loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }
    assert_eq!(count, 30);
    println!("Success!");
}