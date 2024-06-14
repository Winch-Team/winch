use clap::Args;

#[derive(Args)]
pub struct UpgradeArgs {
    pub package: Option<String>,
    #[arg(short, long)]
    pub repo: Option<String>, // Upgrade all packages in a specific repo
}