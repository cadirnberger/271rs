fn lcs(s1: &str, s2: &str) -> String {
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let n = b1.len();
    let m = b2.len();

    // DP table: dp[i][j] = length of LCS of s1[..i] and s2[..j]
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            if b1[i] == b2[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    // Backtrack to build the actual LCS string
    let mut i = n;
    let mut j = m;
    let mut result = Vec::<u8>::new();

    while i > 0 && j > 0 {
        if b1[i - 1] == b2[j - 1] {
            // Characters match → part of LCS
            result.push(b1[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    // result is reversed due to backtracking
    result.reverse();
    String::from_utf8(result).unwrap()
}

fn main() {
    let mut ss = std::env::args();
    let _ = ss.next(); // skip program name

    let s1 = ss.next().unwrap();
    let s2 = ss.next().unwrap();
    dbg!(lcs(&s1, &s2));
}
