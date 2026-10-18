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

    /// Gives u stats
    #[arg(
        long = "stats"
    )]
    stats: bool,

    /// Heartbeat Details
    #[arg(
        long = "heartbeat"
    )]
    heartbeat: Option<String>,

    /// Gives details of your projects
    #[arg(
        long = "project-details"
    )]
    project_details: Option<String>,




    // -- Global Hackatime Stuff --

    /// How many people are currently hacking?
    #[arg(
        long = "current-project"
    )]
    current_project: bool,

    /// How many banned users?
    #[arg(
        long = "banned"
    )]
    banned: bool,

    /// Leaderboard Status
    #[arg(
        long = "leaderboard"
    )]
    leaderboard: Option<String>,

    /// Trust value of user
    #[arg(
        long = "trust-value",
        default_value = "self"
    )]
    trust_value: Option<String>,






    // --- OTHERS ---

    /// Gives a shields.io badge uri for your project
    #[arg(
        long = "badge"
    )]
    badge: Option<String>,

    /// Lookup user
    ///
    /// Usage: --lookup [SLACK_ID / EMAIL]
    #[arg(
        long = "lookup"
    )]
    lookup: Option<String>,


    // --- HACKATUI SPECIFIC STUFF ---
    #[arg(
        long = "no-widget"
    )]
    no_widget: bool,
}


impl CliArgs {
    // checks if there is no cli args given

    /// Check if no cli arguments given
    ///
    /// returns -> bool
    pub(crate) fn is_empty(&self) -> bool {
        // Wakatime Compatible Commands
        if     self.current_user
            || self.summary
            || self.status
            || self.waka_stats
        {
            return false;
        }

        // Hackatime specific stuff
        if     self.stats
            || self.heartbeat.is_some()
            || self.project_details.is_some()
        {
            return false;
        }

        // Global Hackatime Stuff
        if     self.current_project
            || self.banned
            || self.leaderboard.is_some()
            || self.trust_value.is_some()
        {
            return false;
        }

        if     self.badge.is_some()
            || self.lookup.is_some()
        {
            return false;
        }

        true
    }
}

// q w e r t y u i o p
//  a s d f g h j k l
//   z x c v b n m

