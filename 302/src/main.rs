use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} file1 file2", args[0]);
        std::process::exit(1);
    }

    let a = file_to_lines(&args[1]);
    let b = file_to_lines(&args[2]);

    let script = classic_diff(&a, &b);
    print!("{}", script);
}

fn file_to_lines(fname: &str) -> Vec<String> {
    fs::read_to_string(fname)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

// ======== LCS Table ==========
fn lcs_table(a: &[String], b: &[String]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    dp
}

// ======== Produce classic diff operations ==========
fn classic_diff(a: &[String], b: &[String]) -> String {
    let dp = lcs_table(a, b);

    let mut i = a.len();
    let mut j = b.len();

    let mut ops = Vec::<String>::new();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
            // unchanged
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            // addition
            ops.push(format!("{}a{}", i, j));
            ops.push(format!("> {}", b[j - 1]));
            j -= 1;
        } else if i > 0 && (j == 0 || dp[i][j - 1] < dp[i - 1][j]) {
            // deletion
            ops.push(format!("{}d{}", i, j));
            ops.push(format!("< {}", a[i - 1]));
            i -= 1;
        }
    }

    ops.reverse();
    ops.join("\n") + "\n"
}

