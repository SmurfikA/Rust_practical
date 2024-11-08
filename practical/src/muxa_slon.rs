use itertools::Itertools;
fn main() {
    let numbers = (1..=8).collect::<Vec<_>>();
    let permutations = numbers.into_iter().permutations(8);
    for perm in permutations {
        let (m, u, x, a, s, l, o, n) = (perm[0], perm[1], perm[2], perm[3], perm[4], perm[5], perm[6], perm[7]);

        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;
        println!("muxa: {}, a: {}, slon: {}", muxa, a, slon);
        if muxa * a == slon {
            println!("{:04} x {} = {:04}", muxa, a, slon);
        }
    }
}

//змінемо
#[test]
fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];

    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x == m || x == u { continue; }
                for &a in &digits {
                    if a == m || a == u || a == x { continue; }
                    for &s in &digits {
                        if s == m || s == u || s == x || s == a { continue; }
                        for &l in &digits {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for &o in &digits {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for &n in &digits {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    // Обчислення чисел
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;

                                    // Перевірка умови: muxa * a / slon == x
                                    if muxa * a == slon * x {
                                        println!("  {}", muxa);
                                        println!("{}        {}", x, a);
                                        println!("  ------");
                                        println!("   {}", slon);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}