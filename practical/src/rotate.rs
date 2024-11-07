fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let n = ((n % len) + len) % len;
    let split_index = len - n;


    let (left, right) = s.split_at(split_index as usize);
    format!("{}{}", right, left)
}

#[test]
fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(rotate(s.clone(), *n), exp.to_string());
    });
}