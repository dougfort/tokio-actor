use anyhow::Error;

mod actor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handle = actor::MyActorHandle::new();
    let id = handle.get_unique_id().await;
    println!("id = {}", id);

    Ok(())
}
