use loco_rs::{
    environment::{Environment, resolve_from_env},
    app::AppContext,
    db,
    bgworker,
    cache,
    storage::{self, Storage},
    mailer::EmailSender,
    config,
};
use anyhow::Result;
use tracing::warn;

pub fn get_environment() -> Environment {
    let env_str = resolve_from_env();
    Environment::from(env_str)
}

/// Initializes the application context by loading configuration and
/// establishing connections.
///
/// # Errors
/// When has an error to create DB connection.
pub async fn create_context(environment: &Environment) -> Result<AppContext> {
    let config = environment.load()?;

    if config.logger.pretty_backtrace {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1");
        }
        warn!(
            "pretty backtraces are enabled (this is great for development but has a runtime cost \
             for production. disable with `logger.pretty_backtrace` in your config yaml)"
        );
    }

    let db = db::connect(&config.database).await?;

    let mailer = if let Some(cfg) = config.mailer.as_ref() {
        create_mailer(cfg)?
    } else {
        None
    };

    let queue_provider = bgworker::create_queue_provider(&config).await?;
    let ctx = AppContext {
        environment: environment.clone(),
        db,
        queue_provider,
        storage: Storage::single(storage::drivers::null::new()).into(),
        cache: cache::Cache::new(cache::drivers::null::new()).into(),
        config,
        mailer,
    };
    Ok(ctx)
}

/// Initializes an [`EmailSender`] based on the mailer configuration settings
/// ([`config::Mailer`]).
fn create_mailer(config: &config::Mailer) -> Result<Option<EmailSender>> {
    if config.stub {
        return Ok(Some(EmailSender::stub()));
    }
    if let Some(smtp) = config.smtp.as_ref() {
        if smtp.enable {
            return Ok(Some(EmailSender::smtp(smtp)?));
        }
    }
    Ok(None)
}
