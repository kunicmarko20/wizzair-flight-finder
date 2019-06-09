 pub struct MetadataClient;

 const METADATA_URL: &str = "https://wizzair.com/static/metadata.json";

 impl MetadataClient {
     pub fn api_url() -> String {
         let metadata = reqwest::get(METADATA_URL)
             .expect("Failed to fetch the current metadata.")
             .text()
             .expect("Failed to deserialize the metadata.");

         let mut metadata = metadata.chars();

         // https://tools.ietf.org/html/rfc7159#section-8.1
         // Implementations MUST NOT add a byte order mark to the beginning
         //
         // But hey, who follows standards?
         metadata.next();

         let metadata: Metadata = serde_json::from_str(metadata.as_str()).expect("Unable to deserialize metadata.");

         metadata.api_url
     }
 }

 #[derive(Deserialize, Debug)]
 struct Metadata {
     #[serde(rename = "apiUrl")]
     api_url: String,
 }
