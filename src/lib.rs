pub mod decode_utf8;

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use decode_utf8::*;
    
    #[test]
    fn decodes_demo_utf8_txt() {
        let demo_utf8_txt = include_bytes!("../tst-dat/demo-utf8.txt");
        
        let text_content = from_utf8(demo_utf8_txt).unwrap();
        let mut iter = decode_utf8(demo_utf8_txt);
        
        for expected in text_content.chars() {
            if let Some(got) = iter.next_char() {
                assert_eq!(got, expected);
            } else {
                println!("{:?}", iter.status);
                unreachable!();
            }
        }
        
        assert!(iter.next_char().is_none());
    }
    
    #[test]
    fn categorizes_ascii() {
        let bytes = [0x0A, 0x20, 0x2D, 0x32, 0x5A, 0x7A];
        let mut iter = decode_utf8(&bytes);
        
        assert_eq!(iter.next_char_and_category().unwrap().1, CONTROL);
        assert_eq!(iter.next_char_and_category().unwrap().1, SPACE_SEPERATOR);
        assert_eq!(iter.next_char_and_category().unwrap().1, DASH_PUNCTUATION);
        assert_eq!(iter.next_char_and_category().unwrap().1, DECIMAL_NUMBER);
        assert_eq!(iter.next_char_and_category().unwrap().1, UPPERCASE_LETTER);
        assert_eq!(iter.next_char_and_category().unwrap().1, LOWERCASE_LETTER);
        assert!(iter.next_char_and_category().is_none());
    }
}
