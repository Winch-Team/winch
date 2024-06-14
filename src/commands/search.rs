use clap::Args;

#[derive(Args)]
pub struct SearchArgs {
    pub package: String,
    #[arg(short, long)]
    pub local: bool,
}