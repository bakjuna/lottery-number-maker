use crate::Error;
use axum::async_trait;
#[cfg(test)]
use mockall::automock;
use rand::Rng;
use std::{collections::HashMap, sync::Arc};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait LotteryServiceTrait {
    fn is_distributed(&self, numbers: &[i32]) -> bool;
    fn find_most_frequent_number(&self, numbers: &[i32]) -> Option<i32>;
    fn generate_number(&self) -> Option<Vec<i32>>;
}

pub type DynLotteryService = Arc<dyn LotteryServiceTrait + Send + Sync>;
#[derive(Debug, Clone)]
pub struct LotteryService;

#[async_trait]
impl LotteryServiceTrait for LotteryService {
    fn is_distributed(&self, numbers: &[i32]) -> bool {
        if numbers.len() != 6 {
            return false;
        }

        let diffs: Vec<i32> = numbers.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
        let most_frequent_diff = self
            .find_most_frequent_number(&diffs)
            .ok_or(Error::NoFrequentNumber)
            .unwrap();

        diffs
            .into_iter()
            .filter(|&w| w == most_frequent_diff)
            .count()
            >= 3
    }

    fn find_most_frequent_number(&self, numbers: &[i32]) -> Option<i32> {
        let mut frequency_map = HashMap::new();

        for &num in numbers {
            *frequency_map.entry(num).or_insert(0) += 1;
        }

        let most_frequent = frequency_map.iter().max_by_key(|&(_, &count)| count);

        most_frequent.map(|(&number, _)| number)
    }

    fn generate_number(&self) -> Option<Vec<i32>> {
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
}
