//! summary: clap で CLI 定義
//! path: xtask/src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "Generate WorkspaceLayout.md at repo root")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// ルートに WorkspaceLayout.md を生成します
    Layout {
        /// 走査の起点（未指定ならリポジトリルート推定）
        #[arg(long)]
        root: Option<String>,
        /// カンマ区切りの除外名（ディレクトリ/ファイル名一致）
        #[arg(long, value_delimiter = ',', num_args = 0..)]
        exclude: Vec<String>,
        /// 最大深さ（未指定で無制限、ルートを深さ0としてカウント）
        #[arg(long)]
        max_depth: Option<usize>,
        /// ファイル名をこの文字数で省略（0で無効）
        #[arg(long, default_value_t = 0)]
        truncate: usize,
        /// .gitignore を尊重（デフォルト: true）
        #[arg(long, default_value_t = true)]
        respect_gitignore: bool,
    },
}
