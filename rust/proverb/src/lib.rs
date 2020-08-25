pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return "".to_string();
    }

    let mut result = "".to_string();

    for i in 0..(list.len() - 1) {
        result += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
    }

    format!("{}And all for the want of a {}.", result, list[0])
}
