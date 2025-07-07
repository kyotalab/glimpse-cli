use anyhow::Result;

use crate::{generate_operation_id, Cli, Commands};

pub fn dispatch(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Snapshot {
            resource_type,
            resource_id,
            reason,
            ticket,
        } => {
            println!("📸 スナップショット取得を開始します...");
            println!("リソース種別: {}", resource_type);
            println!("リソースID: {}", resource_id);
            println!("作業理由: {}", reason);
            if let Some(ticket) = ticket {
                println!("チケット番号: {}", ticket);
            }

            // 操作ID生成
            let operation_id = generate_operation_id();
            println!("操作ID: {}", operation_id);

            // TODO: 実際のスナップショット取得処理
            println!("✅ スナップショット取得完了（仮実装）");
        }
        Commands::Complete { format, output } => {
            println!("📋 エビデンス生成を開始します...");
            println!("出力形式: {}", format);
            if let Some(output) = output {
                println!("出力先: {}", output.display());
            }

            // TODO: 実際のエビデンス生成処理
            println!("✅ エビデンス生成完了（仮実装）");
        }
        Commands::Status => {
            println!("📊 操作状況:");
            println!("現在進行中の操作はありません（仮実装）");
        }
    }

    Ok(())
}
