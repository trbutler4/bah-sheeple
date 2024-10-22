fn needs_encoding(byte: u8) -> bool {
    !(byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~'))
}

pub fn encode_string(s: String) -> String {
    let mut dst = String::new();

    for &byte in s.as_bytes() {
        if needs_encoding(byte) {
            dst.push('%');
            dst.push_str(&format!("{:02X}", byte));
        } else {
            dst.push(byte as char);
        }
    }

    dst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_string() {
        let test_cases = vec![
            ("Ladies + Gentlemen", "Ladies%20%2B%20Gentlemen"),
            ("An encoded string!", "An%20encoded%20string%21"),
            ("Dogs, Cats & Mice", "Dogs%2C%20Cats%20%26%20Mice"),
            ("☃", "%E2%98%83"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(encode_string(input.to_string()), expected);
        }
    }
}
