use std::sync::Arc;

use tokio_cron_scheduler::JobScheduler;

use crate::errors::BootError;
use crate::cron::handler::LotteryCronJob;


pub async fn create_cron_jobs() -> Result<JobScheduler, BootError> {
	let sched = JobScheduler::new().await;
	if sched.is_err() {
		return Err(BootError::CronJobInit);
	}
	let sched = sched.unwrap();

	let latest_lottery_cron_job = Arc::new(LotteryCronJob {} );
	// TODO: Ensure LotteryCronJob is on DynAppState
	let res = sched
		.add(latest_lottery_cron_job.fetch_latest_lottery_winning_numbers().unwrap())
		.await;
	if res.is_err() {
		return Err(BootError::CronJobRun);
	}
	Ok(sched)
}