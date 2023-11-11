use std::collections::HashMap;

use axum::Json;
use rand::Rng;
use super::model::Lottery;
use crate::{Error, Result};
pub async fn handler_lottery() -> Result<Json<Lottery>> {
    println!(" ->> {:<12} - handler-lottery", "GET");
    let generated_numbers = loop {
        let generated_numbers = generate_number().ok_or(Error::NotFoundError).unwrap();
        if is_distributed(&generated_numbers) {
            continue;
        }
		break generated_numbers;
    };
    let lottery: Lottery = Lottery {
        numbers: generated_numbers,
    };
    let res: Json<Lottery> = Json(lottery);
    Ok(res)
}

fn is_distributed(numbers: &Vec<i32>) -> bool {
    if numbers.len() != 6 {
        return false;
    }

    let diffs: Vec<i32> = numbers.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
    let most_frequent_diff = find_most_frequent_number(&diffs)
        .ok_or(Error::NoFrequentNumber)
        .unwrap();

    println!("{:?}", diffs);
    diffs
        .into_iter()
        .filter(|&w| w == most_frequent_diff)
        .count()
        >= 3
}

fn find_most_frequent_number(numbers: &[i32]) -> Option<i32> {
    let mut frequency_map = HashMap::new();

    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let most_frequent = frequency_map.iter().max_by_key(|&(_, &count)| count);

    match most_frequent {
        Some((&number, _)) => Some(number),
        None => None,
    }
}

fn generate_number() -> Option<Vec<i32>> {
    let mut rng = rand::thread_rng();

    Some((0..6).fold(vec![], |mut acc, _| {
        loop {
            let throw = rng.gen_range(1..=45);
            if acc.contains(&throw) {
                continue;
            }
            acc.insert(acc.len(), throw);
            break;
        }
        acc.sort();
        acc
    }))
}
