// pub fn is_leap_year(year: u64) -> bool {
//     if year % 4 != 0 {
//         return false;
//     }

//     if year % 100 == 0 {
//         return year % 400 == 0;
//     }

//     true
// }


// https://exercism.io/tracks/rust/exercises/leap/solutions/3bbc5d781f8d42dc9bc07ea3d6119614
pub fn is_leap_year(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
