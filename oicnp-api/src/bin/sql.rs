use oicnp_core::{
    services::find_nodes,
    DB, establish_connection,
};

#[tokio::main]
async fn main() {
    let db = DB.get_or_init(establish_connection).await;

    let filters: Vec<String> = vec![];
    let a = find_nodes(db, "", "", &filters, "", "", 0, 10)
        .await;
    
}