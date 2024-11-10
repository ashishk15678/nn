#[cfg(test)]
fn test(i: u64, j: u64) -> u64 {
    i + j
}

#[test]
fn testing_test() {
    let result = test(2, 2);
    assert_eq!(result, 4);
}
