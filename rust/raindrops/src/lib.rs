// pub fn raindrops(n: u32) -> String {
//     let mut result = "".to_string();

//     if n % 3 == 0 {
//         result += "Pling";
//     }

//     if n % 5 == 0 {
//         result += "Plang";
//     }

//     if n % 7 == 0 {
//         result += "Plong";
//     }

//     if result.len() == 0 {
//         result += &n.to_string();
//     }

//     result
// }


// https://exercism.io/tracks/rust/exercises/raindrops/solutions/6218001325d246309ca9bda239ae9108
pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    if n % 3 == 0 {
        result.push_str("Pling");
    }

    if n % 5 == 0 {
        result.push_str("Plang");
    }

    if n % 7 == 0 {
        result.push_str("Plong");
    }

    if result.is_empty() {
        result += &n.to_string();
    }

    result
}
