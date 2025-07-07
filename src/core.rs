use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Evidence {
    pub metadata: OperationMetadata,
    pub operation: OperationInfo,
    pub snapshots: SnapshotPair,
    pub analysis: Option<ChangeAnalysis>, // completeコマンド実行後に設定
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationMetadata {
    pub operation_id: String,
    pub timestamp: DateTime<Utc>,
    pub operator: String,
    pub aws_account_id: String,
    pub region: String,
    pub reason: String,
    pub ticket_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfo {
    pub operation_type: String, // "ec2_instance_stop"
    pub target_resource: String, // "i-1234567890abcdef0"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotPair {
    pub before: Option<ResourceSnapshot>,
    pub after: Option<ResourceSnapshot>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceSnapshot {
    pub timestamp: DateTime<Utc>,
    pub resources: HashMap<String, ResourceData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceData {
    pub resource_type: String,
    pub resource_id: String,
    pub attributes: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeAnalysis {
    pub changed_resources: Vec<ResourceChange>,
    pub unchanged_resources: Vec<ResourceInfo>,
    pub summary: ChangeSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceChange {
    pub resource_type: String,
    pub resource_id: String,
    pub change_type: ChangeType,
    pub field_changes: Vec<FieldChange>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeType {
    Modified,
    Added,
    Removed,
    Unchanged,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldChange {
    pub field_path: String,
    pub before: Option<Value>,
    pub after: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub resource_type: String,
    pub resource_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeSummary {
    pub total_resources_monitored: usize,
    pub resources_changed: usize,
    pub resources_unchanged: usize,
    pub operation_duration_seconds: i64,
}

// 操作ID生成関数
pub fn generate_operation_id() -> String {
    let now = Utc::now();
    format!("evidence_{}", now.format("%Y%m%d_%H%M%S"))
}
