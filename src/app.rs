use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    bgworker::{BackgroundWorker, Queue},
    Result,
    config::Config,
};
use migration::Migrator;
use oic_core::entities::prelude::*;
use axum::Router as AxumRouter;
use crate::controllers::home::fallback;

use crate::{controllers, initializers, tasks, workers::downloader::DownloadWorker};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        let app_routes = AppRoutes::with_default_routes()
            .add_routes(controllers::routes())
            .add_routes(controllers::v1::routes());

        app_routes
    }

    async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let router = router.fallback(fallback);
        Ok(router)
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, UserEntity).await?;
        truncate_table(&ctx.db, NoteEntity).await?;
        Ok(())
    }

    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        // db::seed::<UserActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        db::seed::<NoteActiveModel>(&ctx.db, &base.join("notes.yaml").display().to_string()).await?;
        Ok(())
    }
}
