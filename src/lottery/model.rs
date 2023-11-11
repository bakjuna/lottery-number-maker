
use serde::Serialize;


#[derive(Clone, Debug, Serialize)]
pub struct Lottery {
	pub numbers: Vec<i32>
}