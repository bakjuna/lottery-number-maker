use tokio_cron_scheduler::{Job, JobSchedulerError};

use crate::errors::BootError;

pub struct LotteryCronJob {}
impl LotteryCronJob {
    pub fn fetch_latest_lottery_winning_numbers(&self) -> Result<Job, BootError> {
        let job = Job::new("* * * * * *", |_, __| {
            let _url = "https://dhlottery.co.kr/gameResult.do?method=allWinExel&nowPage=1&gubun=byWin&nowPage=&drwNoStart=1&drwNoEnd=1000";
            // TODO: get number, and save it to database
        });
        Self::return_job(job)
    }

    fn return_job(job: Result<Job, JobSchedulerError>) -> Result<Job, BootError> {
        match job {
            Ok(value) => Ok(value),
            Err(err) => Self::lottery_cron_job_error_handler::<Job>(err),
        }
    }

    fn lottery_cron_job_error_handler<T>(err: JobSchedulerError) -> Result<T, BootError> {
        match err {
            tokio_cron_scheduler::JobSchedulerError::StartScheduler => {
                Err(BootError::CronJobInit)
            }
            _ => Err(BootError::CronJobRun),
        }
    }
}
