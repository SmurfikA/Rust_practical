

fn main() {
    if std::env::args().count() < 2 {
        eprintln!("Usage: gray <n>...<n>");
        std::process::exit(1);
    }

    std::env::args().skip(1).for_each(|arg| {
        let n = arg.parse::<u8>().unwrap_or_else(|_| {
            eprintln!("Expected unsigned integer, got: {arg}");
            std::process::exit(1);
        });
        let codes = gray(n);
        println!("{arg} -> {:?}", codes);
    });
}

fn gray(n: u8) -> Vec<String> {
    let size = 1 << n; // 2^n кількість кодів
    let mut result = Vec::with_capacity(size);

    for i in 0..size {
        let gray_code = i ^ (i >> 1); // Отримуємо Gray-код
        // Форматуємо Gray-код у бінарний рядок, доповнюючи до n символів
        result.push(format!("{:0width$b}", gray_code, width = n as usize));
    }

    result
}

#[test]
mod tests {
    use super::gray;

    #[test]
    fn test_gray() {
        let test_data = [
            (0, vec![""]),
            (1, vec!["0", "1"]),
            (2, vec!["00", "01", "11", "10"]),
            (3, vec!["000", "001", "011", "010", "110", "111", "101", "100"]),
            (4, vec![
                "0000", "0001", "0011", "0010", "0110", "0111", "0101", "0100",
                "1100", "1101", "1111", "1110", "1010", "1011", "1001", "1000"
            ]),
        ];

        test_data.iter().for_each(|(n, out)| assert_eq!(gray(*n), *out));
    }
}
