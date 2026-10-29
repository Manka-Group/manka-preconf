use tonic::transport::{Channel, ClientTlsConfig};

pub mod sugar {
    tonic::include_proto!("sugar.v1");
}
use sugar::{sugar_client::SugarClient, update::Update, Filter, SubscribeRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = std::env::var("MANKA_API_KEY")?;
    let channel = Channel::from_static("https://fra-preconf.manka.wtf:12349")
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;

    let jupiter = bs58::decode("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").into_vec()?;
    let mut request = tonic::Request::new(SubscribeRequest {
        filters: [
            (
                "jupiter".to_string(),
                Filter { accounts: vec![jupiter.into()], ..Default::default() },
            ),
            (
                "launches".to_string(),
                Filter { actions: vec!["create".to_string()], ..Default::default() },
            ),
        ]
        .into(),
    });
    request.metadata_mut().insert("x-api-key", key.parse()?);

    let mut updates = SugarClient::new(channel).subscribe(request).await?.into_inner();
    while let Some(update) = updates.message().await? {
        match update.update {
            Some(Update::Slot(s)) => println!("slot {} leader {}", s.slot, bs58::encode(&s.leader).into_string()),
            Some(Update::Transaction(tx)) => println!(
                "slot {} #{} {} matched {:?}",
                tx.slot,
                tx.index,
                bs58::encode(&tx.signature).into_string(),
                tx.filters
            ),
            None => {}
        }
    }
    Ok(())
}
