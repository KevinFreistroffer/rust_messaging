// use enums::{Message, MessageType, MessageFrom};
// use enums::Message::{File, Image, Text};
// use structs::messages::{
//     file::FileMessagePackage, image::ImageMessagePackage, text::TextMessagePackage,
// };
use mongodb::{
    bson::{doc, Document}, options::{ClientOptions, ResolverConfig, ServerApi, ServerApiVersion}, Client, Collection, Database
};

pub async fn get_database() -> Result<Database, mongodb::error::Error> {
    let uri = "mongodb+srv://kfreistroffer:u4pntnFOxFIrpbjq@cluster0.ycyhsgq.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
    let mut client_options = ClientOptions::parse(uri).resolver_config(ResolverConfig::cloudflare()).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    let database = client.database("rust");
    Ok(database)
}

pub async fn get_collection(
    collection: &str,
) -> Result<Collection<Document>, mongodb::error::Error> {
    let database = get_database().await?;
    let collection = database.collection(collection);
    Ok(collection)
}
