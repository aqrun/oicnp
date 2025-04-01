use oic_core::{
    AppContext,
    services::user::check_user_has_role,
    uuid,
};
use anyhow::Result;

pub async fn run(ctx: &AppContext) -> Result<()> {
    // let _ = check_user_has_role(&ctx.db, 1, "author").await?;
    // for _i in 0..10 {
    //     println!("{}", oid!());
    // }

    let id1 = uuid!();
    let id2 = uuid!("i");
    let id3 = uuid!("note", 22);

    println!("id1 {id1}, id2 {id2}, id3 {id3}");
    Ok(())
}