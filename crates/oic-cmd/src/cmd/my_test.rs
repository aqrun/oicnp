use oic_core::{
    AppContext,
    services::user::check_user_has_role,
};
use anyhow::Result;

pub async fn run(ctx: &AppContext) -> Result<()> {
    let _ = check_user_has_role(&ctx.db, 1, "author").await?;
  Ok(())
}