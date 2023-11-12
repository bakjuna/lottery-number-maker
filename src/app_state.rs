use std::sync::Arc;

#[cfg(test)]
use mockall::automock;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenv::dotenv;
use crate::{EnvVars, DynLotteryService, lottery::service::LotteryService};
struct AppState {
    db: Pool<Postgres>,
    env: EnvVars,
    lottery_service: DynLotteryService,
}

#[cfg_attr(test, automock)]
pub trait AppStateTrait {
    fn get_db(&self) -> Pool<Postgres>;
    fn get_env(&self) -> EnvVars;
    fn get_lottery_service(&self) -> DynLotteryService;
}

impl AppStateTrait for AppState {
    fn get_db(&self) -> Pool<Postgres> {
        self.db.clone()
    }
    fn get_env(&self) -> EnvVars {
        self.env.clone()
    }

    fn get_lottery_service(&self) -> DynLotteryService {
        self.lottery_service.clone()
    }
}

pub type DynAppState = Arc<dyn AppStateTrait + Send + Sync>;

pub async fn create_app_state() -> DynAppState {
    println!("Creating App State...");
    dotenv().ok();
    let envs = EnvVars::new();
    let database_url = format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        "postgres://",
        envs.postgres.user,
        ":",
        envs.postgres.password,
        "@",
        envs.postgres.host,
        ":",
        envs.postgres.port,
        "/",
        envs.postgres.database,
        "?schema=public"
    );
    println!("Connecting Database...");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database Connected");
            pool
        }
        Err(err) => {
            println!("Database not Connected: {:?}", err);
            std::process::exit(1);
        }
    };
    let envs = EnvVars::new();
    let service = Arc::new(LotteryService {}) as DynLotteryService;
	println!("Creating App State Completed");
    Arc::new(AppState {
        db: pool,
        env: envs,
        lottery_service: service,
    }) as DynAppState
}