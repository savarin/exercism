fn bottle(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

fn last(n: u32) -> String {
    match n {
        1 => "it".to_string(),
        _ => "one".to_string(),
    }
}

pub fn verse(n: u32) -> String {
    format!(
        "\
{} of beer on the wall, {} of beer.
Take {} down and pass it around, {} of beer on the wall.\n",
        bottle(n),
        bottle(n),
        last(n),
        bottle(n - 1)
    )
}

pub fn buy() -> String {
    format!(
        "\
No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n"
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = "".to_string();
    let count = start - end;

    for n in 0..count {
        result += &(verse(start - n) + "\n");
    }

    if end == 0 {
        result += &buy();
    } else {
        result += &verse(end);
    }

    result
}
