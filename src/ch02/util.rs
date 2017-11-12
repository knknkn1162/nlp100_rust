/// helper for ch02.16,
pub fn get_split_line_count(size: usize, split_num: usize)->usize {
    let res: usize = size/split_num;
    if size%split_num==0 {res} else {res+1}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_split_line_count(9, 3), 3);
        assert_eq!(get_split_line_count(10, 3), 4);
        assert_eq!(get_split_line_count(11, 3), 4);
    }

    #[test]
    fn test_trim_mut() {
        let mut s = "abc\n\n".to_string();
        trim_mut(&mut s, '\n');

        assert_eq!("abc", s);
    }
}