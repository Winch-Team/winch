use clap::Args;

#[derive(Args)]
pub struct SyncArgs {
    pub repo: Option<String>, // sync one repo or all repos
}