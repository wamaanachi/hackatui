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

    // --- HACKATIME SPECIFIC STUFF ---

    /// What are you currently hacking?
    #[arg(
        long = "current-project"
    )]
    current_project: bool,

    /// Leaderboard Status
    #[arg(
        long = "leaderboard"
    )]
    leaderboard: String,

    /// Heartbeat Details
    #[arg(
        long = "heartbeat"
    )]
    heartbeat: String,
}


impl CliArgs {
    // checks if there is no cli args given

    /// Check if no cli arguments given
    ///
    /// returns -> bool
    pub(crate) fn is_none(&self) -> bool {
        // Wakatime Compatible Commands
        if self.current_user || self.summary ||self.status || self.stats {
            return false;
        }

        // Hackatime Specific Commands
        if self.current_project {
            return false;
        }

        if !self.leaderboard.is_empty() {
            return false;
        }

        if !self.heartbeat.is_empty() {
            return false;
        }

        true
    }
}

// q w e r t y u i o p
//  a s d f g h j k l
//   z x c v b n m

