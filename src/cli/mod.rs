pub mod dispatch;
pub use dispatch::*;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "glimpse")]
#[command(about = "AWS操作エビデンス自動生成ツール")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 操作前スナップショット取得
    Snapshot {
        /// リソース種別 (ec2)
        resource_type: String,
        /// リソースID (i-1234567890abcdef0)
        resource_id: String,
        /// 作業理由
        #[arg(long)]
        reason: String,
        /// チケット番号
        #[arg(long)]
        ticket: Option<String>,
    },
    /// 操作後スナップショット取得・エビデンス生成
    Complete {
        /// 出力形式 (json,csv,pdf)
        #[arg(long, default_value = "json")]
        format: String,
        /// 出力ディレクトリ
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// 操作状況確認
    Status,
}
