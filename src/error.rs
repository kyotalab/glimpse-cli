use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("AWS SDK error: {0}")]
    AwsSdk(#[from] aws_sdk_ec2::Error),
 
    #[error("AWS STS error: {0}")]
    AwsSts(#[from] aws_sdk_sts::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("No active operation found")]
    NoActiveOperation,

    #[error("Resource not found: {resource_id}")]
    ResourceNotFound { resource_id: String },

    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },

    #[error("Home directory not found")]
    HomeNotFound,

    #[error("Invalid resource type: {resource_type}")]
    InvalidResourceType { resource_type: String },
}

pub type Result<T> = std::result::Result<T, Error>;

