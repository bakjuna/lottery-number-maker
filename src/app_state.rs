use std::sync::Arc;

#[cfg(test)]
use mockall::automock;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenv::dotenv;
use crate::{EnvVars, DynLotteryService, lottery::service::LotteryService};

module! {
	AppModule {
			components = [LotteryService],
			providers = []
	}
}

#[derive(Clone)]
struct AppState {
    // db: Pool<Postgres>,
    // env: EnvVars,
    lottery_service: Arc<AppModule>,
}
impl FromRef<AppState> for Arc<AppModule> {
	fn from_ref(app_state: &AppState) -> Arc<AppModule> {
			app_state.module.clone()
	}
}

#[cfg_attr(test, automock)]
pub trait AppStateTrait {
    // fn get_db(&self) -> Pool<Postgres>;
    // fn get_env(&self) -> EnvVars;
    fn get_lottery_service(&self) -> Arc<AppModule>;
}

// impl AppStateTrait for AppState {
//     fn get_db(&self) -> Pool<Postgres> {
//         self.db.clone()
//     }
//     fn get_env(&self) -> EnvVars {
//         self.env.clone()
//     }

//     fn get_lottery_service(&self) -> DynLotteryService {
//         self.lottery_service.clone()
//     }
// }

// pub type DynAppState = Arc<dyn AppStateTrait + Send + Sync>;

pub async fn create_app_state() -> AppState {
    println!("Creating App State...");
		let module = Arc::new(AppModule::builder().build());
    AppState { module }
}