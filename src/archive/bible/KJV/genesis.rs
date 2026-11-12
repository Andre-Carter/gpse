pub const GENESIS_TEXT: &str =
    include_str!("XGPSE/archive/KJV/Genesis.md");

#[test]
fn genesis_text_is_embedded() {
    assert!(!GENESIS_TEXT.is_empty());
}