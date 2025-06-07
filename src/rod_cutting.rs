pub fn rod_cutting(prices: &[i32], n: usize) -> i32 {
    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        let mut max_val = i32::MIN;
        for j in 0..i {
            max_val = max_val.max(prices[j] + dp[i - j - 1]);
        }
        dp[i] = max_val;
    }

    dp[n]
}

pub fn rod_cutting_naive(prices: &[i32], n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut max_val = i32::MIN;

    for i in 0..n {
        let cost = prices[i] + rod_cutting_naive(prices, n - i - 1);
        max_val = max_val.max(cost);
    }

    max_val
}
