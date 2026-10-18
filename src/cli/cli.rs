use clap::Parser;


#[derive(Clone, Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
)]
pub struct CliArgs {
    // --- WAKATIME COMPATIBLE

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
        long = "waka-stats",
    )]
    waka_stats: bool,

    // --- HACKATIME SPECIFIC STUFF ---

    /// How many people are currently hacking?
    #[arg(
        long = "current-project"
    )]
    current_project: bool,

    /// Leaderboard Status
    #[arg(
        long = "leaderboard"
    )]
    leaderboard: Option<String>,

    /// Heartbeat Details
    #[arg(
        long = "heartbeat"
    )]
    heartbeat: Option<String>,

    #[arg(
        long = "stats"
    )]
    stats: bool,

    // -- Global Hackatime Stuff --

    /// How many people are currently hacking?
    #[arg(
        long = "current-project"
    )]
    current_project: bool,

    /// Leaderboard Status
    #[arg(
        long = "leaderboard"
    )]
    leaderboard: Option<String>,

    /// How many banned users?
    #[arg(
        long = "banned"
    )]
    banned: bool,

    /// Trust value of user
    #[arg(
        long = "trust-value",
        default_value = "self"
    )]
    trust_value: Option<String>,

    // --- OTHERS ---
    #[arg(
        long = "badge"
    )]
    badge: Option<String>,
}


impl CliArgs {
    // checks if there is no cli args given

    /// Check if no cli arguments given
    ///
    /// returns -> bool
    pub(crate) fn is_empty(&self) -> bool {
        // Wakatime Compatible Commands
        if self.current_user || self.summary || self.status || self.waka_stats {
            return false;
        }

        // Global hackatime stuff
        if self.current_project || self.banned || self.trust_value.is_none() {
            return false;
        }

        if self.stats {
            return false;
        }

        if     self.leaderboard.is_some()
            || self.heartbeat  .is_some()
            || self.badge      .is_some()
        {
            return false;
        }

        true
    }
}

// q w e r t y u i o p
//  a s d f g h j k l
//   z x c v b n m

