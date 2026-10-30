use crate::text;

#[test]
fn converts_byte_columns_to_utf16_units() {
    let lines = text::lines("ñ\n😀x\nplain");

    assert_eq!(text::utf16_column(&lines, 0, 0), 0);
    assert_eq!(text::utf16_column(&lines, 0, 2), 1);
    assert_eq!(text::utf16_column(&lines, 1, 4), 2);
    assert_eq!(text::utf16_column(&lines, 2, 3), 3);
    assert_eq!(text::utf16_column(&lines, 9, 5), 5);
}
