 pub struct MetadataClient;

 const METADATA_URL: &str = "https://wizzair.com/static_fe/metadata.json";

 impl MetadataClient {
     pub fn api_url() -> String {
         let metadata = reqwest::get(METADATA_URL)
             .expect("Failed to fetch the current metadata.")
             .text()
             .expect("Failed to deserialize the metadata.");

         let metadata: Metadata = serde_json::from_str(metadata.chars().as_str())
             .expect("Unable to deserialize metadata.");

         metadata.api_url
     }
 }

 #[derive(Deserialize, Debug)]
 struct Metadata {
     #[serde(rename = "apiUrl")]
     api_url: String,
 }
