fn factorial(n: u64 = 0) -> u64 {
    if n == 0 { return 0; }
    if n < 2 { return n; }
    return n + factorial(n - 1);
}

