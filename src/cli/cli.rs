use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
)]
pub struct CliArgs {
    /// Gives the current user
    #[arg(
        short = 'u',
        long = "user"
    )]
    current_user: bool,

    /// Gives a Summary of your hacking
    #[arg(
        short = 's',
        long = "summary"
    )]
    summary: bool,

    /// Gives Status for today
    #[arg(
        long = "status",
    )]
    status: bool,

    /// Gives stats for the past week
    #[arg(
        short = 's',
        long = "stats",
    )]
    stats: bool,
}

impl CliArgs {
    // checks if there is no cli args given
    pub(crate) fn is_none(&self) -> bool {
        // Wakatime Compatible Commands
        if self.current_user {
            return false
        }
        if self.status {
            return false
        }
        if self.summary {
            return false
        }
        if self.stats {
            return false
        }

        true
    }
}
