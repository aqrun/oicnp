use std::time::Duration;

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use oic_core::entities::prelude::*;

pub struct DownloadWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DownloadWorkerArgs {
    pub user_guid: String,
}

#[async_trait]
impl BackgroundWorker<DownloadWorkerArgs> for DownloadWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
    async fn perform(&self, args: DownloadWorkerArgs) -> Result<()> {
        // TODO: Some actual work goes here...
        println!("================================================");
        println!("Sending payment report to user {}", args.user_guid);

        sleep(Duration::from_millis(2000)).await;

        let all = UserEntity::find()
            .all(&self.ctx.db)
            .await
            .map_err(Box::from)?;
        for user in &all {
            println!("user: {}", user.uid);
        }
        println!("================================================");
        Ok(())
    }
}
