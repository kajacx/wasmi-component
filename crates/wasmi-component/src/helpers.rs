pub fn round_up(value: usize, multiple: usize) -> usize {
    (value + multiple - 1) / multiple * multiple
}

#[test]
fn test_round_up() {
    assert_eq!(round_up(7, 5), 10);
    assert_eq!(round_up(8, 5), 10);
    assert_eq!(round_up(9, 5), 10);
    assert_eq!(round_up(10, 5), 10);
    assert_eq!(round_up(11, 5), 15);
    assert_eq!(round_up(12, 5), 15);
}
