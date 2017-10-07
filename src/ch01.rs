/// ch1.00 reverse "stressed"
///
/// ```
/// chs = str.chars(): Chars // An iterator over the chars of a string slice.
/// rev = chs.rev(): std::iter::Rev // Reverses an iterator's direction.
/// ans = rev.collect::<String>() // Transforms an iterator into a collection.
/// ```
///
pub fn reverse(str: &str)-> String {
    str.chars().rev().collect::<String>()
}

/// ch1.01 extract chars of odd position from "パタトクカシーー"
///
pub fn extract<Pred>(str: &str, pred: Pred)-> String
    where Pred : Fn(usize) -> bool
{
    str.chars()
        .enumerate()
        .filter(|&(idx, _)| pred(idx))
        .map(|(_, elem)| elem)
        .collect::<String>()
}

/// ch1.02 concat two words alternatively. (e.g. "abc", "def" => "adbecf")
///
pub fn join_alt(str1: &str, str2: &str)-> String {
    str1.chars()
        .zip(str2.chars())
        .map(|(ch1, ch2)| format!("{}{}", ch1,ch2))
        .collect::<String>()
}
