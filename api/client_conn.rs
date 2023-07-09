use mongodb::{Client, Database};
use std::sync::OnceLock;

const DB_NAME: &'static str = "vault";

// make a global static CLIENT that is a singleton. Set one time and then just returned.
static CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn get_conn() -> mongodb::error::Result<Database> {
    let client = if let Some(client) = CLIENT.get() {
        client
    } else {
        let password = std::env::var("MONGO_PASSWORD")
            .expect("Enviornment variable MONGO_PASSWORD not found.");
        let password = urlencoding::encode(&password);
        let uri = format!("mongodb+srv://vivekyadav7272:{password}@fortress.yupzddu.mongodb.net/?retryWrites=true&w=majority");
        let client = Client::with_uri_str(uri).await?;
        CLIENT.set(client).expect("Couldn't set the client");
        CLIENT
            .get()
            .expect("VERY WEIRD: Couldn't get the client after setting it")
    };
    Ok(client.database(DB_NAME))
}
