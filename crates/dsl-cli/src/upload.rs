//! Upload rendered diagram SVG to MinIO (or any S3-compatible store).
//!
//! Content-addressed: the object key is the SHA-256 of the bytes plus the
//! extension. Same diagram from multiple problems = single object,
//! deduplicated automatically.

use s3::Bucket;
use s3::creds::Credentials;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Uploader {
    bucket: Box<Bucket>,
    public_base: String,
}

impl Uploader {
    /// Build an uploader from environment variables. Returns `Ok(None)` if any
    /// required var is missing — caller should treat that as "uploads disabled"
    /// and fall back to embedding SVG inline (or skipping).
    pub fn from_env() -> Result<Option<Self>, String> {
        let endpoint = match std::env::var("S3_ENDPOINT_URL") {
            Ok(v) if !v.is_empty() => v,
            _ => return Ok(None),
        };
        let bucket_name = std::env::var("S3_BUCKET").unwrap_or_else(|_| "diagrams".into());
        let public_base = std::env::var("S3_PUBLIC_BASE")
            .unwrap_or_else(|_| format!("{}/{}", endpoint.trim_end_matches('/'), bucket_name));
        let access_key = std::env::var("MINIO_ACCESS_KEY")
            .or_else(|_| std::env::var("AWS_ACCESS_KEY_ID"))
            .map_err(|_| "MINIO_ACCESS_KEY (or AWS_ACCESS_KEY_ID) required".to_string())?;
        let secret_key = std::env::var("MINIO_SECRET_KEY")
            .or_else(|_| std::env::var("AWS_SECRET_ACCESS_KEY"))
            .map_err(|_| "MINIO_SECRET_KEY (or AWS_SECRET_ACCESS_KEY) required".to_string())?;
        let region = s3::Region::Custom {
            region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".into()),
            endpoint,
        };
        let creds = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)
            .map_err(|e| format!("creds: {e}"))?;
        let mut bucket =
            Bucket::new(&bucket_name, region, creds).map_err(|e| format!("bucket: {e}"))?;
        bucket.set_path_style();
        Ok(Some(Self {
            bucket,
            public_base,
        }))
    }

    /// Upload `bytes` under a content-hash key. Returns the public URL.
    /// Idempotent: re-uploading the same bytes just overwrites with itself.
    pub async fn put(
        &self,
        bytes: &[u8],
        extension: &str,
        content_type: &str,
    ) -> Result<String, String> {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash = hex::encode(hasher.finalize());
        let key = format!("{hash}.{extension}");
        let response = self
            .bucket
            .put_object_with_content_type(&key, bytes, content_type)
            .await
            .map_err(|e| format!("upload {key}: {e}"))?;
        let status = response.status_code();
        if !(200..300).contains(&status) {
            return Err(format!("upload {key}: HTTP {status}"));
        }
        Ok(format!(
            "{}/{}",
            self.public_base.trim_end_matches('/'),
            key
        ))
    }
}
