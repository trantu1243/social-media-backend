use aws_config::Region;
use aws_sdk_s3::{config::Credentials, Client};
use aws_sdk_s3::primitives::ByteStream;
use dotenv::dotenv;
use uuid::Uuid;
use std::env;

pub struct AwsS3;

impl AwsS3{
    
    pub async fn upload_s3(bytes: Vec<u8>, content_type: &str) -> Result<String, String> {
        dotenv().ok();
        let region = env::var("AWS_REGION").expect("AWS_REGION must be set");
        let access_key = env::var("AWS_ACCESS_KEY").expect("AWS_ACCESS_KEY must be set");
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
        let bucket = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set");
        let s3_client = Self::get_s3_client(region, access_key, secret_key).await;
    
        let file_extension = match content_type {
                    "image/jpeg" => "jpg".to_string(),
                    "image/png" => "png".to_string(),
                    "image/gif" => "gif".to_string(),
                    _ => return Err("Unsupported file type".to_string())
                };
         

        let file_name = format!("avatar-{}.{}", Uuid::new_v4(), file_extension);

        let byte_stream = ByteStream::from(bytes);

        let _resp = s3_client.put_object()
            .bucket(&bucket)
            .key(&file_name)
            .body(byte_stream)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| format!("Failed to upload to S3: {}", e))?;

        let image_url = format!("https://{}.s3.amazonaws.com/{}", bucket, file_name);
        Ok(image_url)
    }

    async fn get_s3_client(region: String, access_key: String, secret_key: String) -> Client {
        let _config = aws_config::load_from_env().await;
        let credentials_provider = Credentials::new(access_key, secret_key, None, None, "custom");
        let config = aws_sdk_s3::Config::builder()
            .region(Region::new(region))
            .credentials_provider(credentials_provider)
            .build();
    
        Client::from_conf(config)
    }
}