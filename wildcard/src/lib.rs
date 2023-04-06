pub fn is_match(s: String, p: String) -> bool {
    fn rec(
        memo: &mut Vec<Vec<Option<bool>>>,
        s: &Vec<char>,
        p: &Vec<char>,
        i: i32,
        j: i32,
    ) -> bool {
        if i == -1 || j == -1 {
            return i == -1 && (j == -1 || p[0..=j as usize].iter().all(|x| *x == '*'));
        }
        let j = j as usize;
        let i = i as usize;
        if memo[i][j].is_none() {
            memo[i][j] = Some(match p[j] {
                '*' => {
                    rec(memo, s, p, i as i32 - 1, j as i32)
                        || rec(memo, s, p, i as i32 - 1, j as i32 - 1)
                        || rec(memo, s, p, i as i32, j as i32 - 1)
                }
                '?' => rec(memo, s, p, i as i32 - 1, j as i32 - 1),
                _ => {
                    if p[j] == s[i] {
                        rec(memo, s, p, i as i32 - 1, j as i32 - 1)
                    } else {
                        false
                    }
                }
            })
        }
        memo[i][j].unwrap()
    }
    rec(
        &mut vec![vec![None; p.len()]; s.len()],
        &s.chars().collect(),
        &p.chars().collect(),
        s.len() as i32 - 1,
        p.len() as i32 - 1,
    )
}
