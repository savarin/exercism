pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = Vec::new();

    for item in factors {
        if *item == 0 {
            continue;
        }

        let mut counter = 1;

        loop {
            let product = item * counter;

            if product < limit {
                if !multiples.contains(&product) {
                    multiples.push(product);
                }
                counter += 1;
                continue;
            }

            break;
        }
    }

    let mut result = 0;

    for item in multiples {
        result += item;
    }

    result
}
