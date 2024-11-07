#[test]
fn test() {
    let (x, y);
    (x, _) = (3, 4);
    [_, y] = [1, 2];
    assert_eq!([x,y], [3, 2]);
    println!("Success!");
}