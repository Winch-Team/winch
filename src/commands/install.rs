use clap::Args;

#[derive(Args)]
pub struct InstallArgs {
    pub package: String, 
    pub repo: Option<String>, 
    #[arg(short, long)]
    pub local: bool,
}