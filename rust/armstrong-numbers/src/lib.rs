pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let s = num.to_string();
    let n = s.len() as u32;
    let mut result = 0;

    for c in s.chars() {
        result += (c as u32 - '0' as u32).pow(n);
    }

    result == num
}
