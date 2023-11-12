use super::model::Lottery;
use crate::{DynAppState, Error, Result};
use axum::{extract::State, Json};

pub async fn handler_lottery(State(data): State<DynAppState>) -> Result<Json<Lottery>> {
    println!(" ->> {:<12} - handler-lottery", "GET");
    println!("{}", data.get_env().postgres.database);
    let generated_numbers = loop {
        let generated_numbers = data
            .get_lottery_service()
            .generate_number()
            .ok_or(Error::NotFoundError)
            .unwrap();
        if data
            .get_lottery_service()
            .is_distributed(&generated_numbers)
        {
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
