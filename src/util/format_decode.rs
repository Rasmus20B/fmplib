
pub fn get_path_int(bytes : &[u8]) -> usize {
    match bytes.len() {
        1 => bytes[0] as usize,
        2 => 0x80 + ((bytes[0] as usize) & 0x7f << 8) + bytes[1] as usize,
        _ => 0
    }
}

pub fn get_int(bytes: &[u8]) -> usize {
    return match bytes.len() {
        1 => bytes[0] as usize,
        2 => ((bytes[0] as usize) << 8) + (bytes[1] as usize),
        4 => (get_int(&bytes[0..2]) << 16) + get_int(&bytes[2..4]),
        _ => 0
    }
}

pub fn fm_string_decrypt(bytes: &[u8]) -> String {
    match String::from_utf8(bytes
                                 .into_iter()
                                 .map(|c| c ^ 0x5A)
                                 .collect::<Vec<u8>>()) {
        Ok(v) => v.to_string(),
        Err(e) => "value not utf-8.".to_string()
    }
}
#[cfg(test)]
mod tests {
    use crate::encoding_util::*;
    #[test]
    fn int_testing() {
        assert_eq!(get_path_int(&[128, 138]), 266);
        assert_eq!(get_path_int(&[128, 138]), 266);
        assert_eq!(get_path_int(&[]), 0);
    }

    #[test]
    fn string_testing() {
        assert_eq!(fm_string_decrypt(&[0x7e, 0x22]), "$x");
        assert_eq!(fm_string_decrypt(&[0x7e, 0x23]), "$y");
        assert_eq!(fm_string_decrypt(&[0x32, 0x3f, 0x36, 0x36, 0x35]), "hello");
        assert_eq!(fm_string_decrypt(&[]), "");
    }
}

