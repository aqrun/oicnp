use zino::prelude::*;

pub fn async_job_scheduler() -> AsyncJobScheduler {
    let scheduler = AsyncJobScheduler::new();
    
    scheduler
}