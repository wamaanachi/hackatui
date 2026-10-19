// this module is for the cli stuff

pub mod cli;

use clap::arg;
use cli::CliArgs;


/// Routes the cli args to their various outputs
pub(crate) fn parse(args: CliArgs) {
    let widget: bool;
    if args.no_widget {
        widget = false;
    } else {
        widget = true;
    }

    waka_parse         (args.clone(), widget);
    hacka_parse        (args.clone(), widget);
    hacka_global_parse (args.clone(), widget);
    hacka_other_parse  (args.clone(), widget);
}


/// Routes the wakatime specific commands to their outputs
fn waka_parse(args: CliArgs, widget: bool) {
    if args.current_user {
        if widget {
            todo!(
                "call nowidget current user"
            )
        } else {
            todo!(
                "call current user"
            )
        }
    }

    if args.summary {
        if widget {
            todo!(
                "call nowidget summary"
            )
        } else {
            todo!(
                "call summary"
            )
        }
    }

    if args.status {
        if widget {
            todo!(
                "call nowidget status"
            )
        } else {
            todo!(
                "call status"
            )
        }
    }

    if args.waka_stats {
        if widget {
            todo!(
                "call nowidget waka stats"
            )
        } else {
            todo!(
                "call waka stats"
            )
        }
    }
}

/// Routes the hackatime specific main commands to their outputs
fn hacka_parse(args: CliArgs, widget: bool) {
    if args.stats {
        if widget {
            todo!(
                "call nowidget stats"
            )
        } else {
            todo!(
                "call stats"
            )
        }
    }

    if let Some(heartbeat) = args.heartbeat {
        todo!(
            "call heartbeat processor"
        )
    }

    if let Some(project_details) = args.project_details {
        todo!(
            "call project details processor"
        )
    }
}

/// Routes the hackatime specific global userbase commands to their outputs
fn hacka_global_parse(args: CliArgs, widget: bool) {
    if args.currently_hacking {
        if widget {
            todo!(
                "call nowidget currently hacking"
            )
        } else {
            todo!(
                "call currently hacking"
            )
        }
    }

    if args.banned {
        if widget {
            todo!(
                "call nowidget banned"
            )
        } else {
            todo!(
                "call banned"
            )
        }
    }

    if let Some(leaderboard) = args.leaderboard {
        todo!(
            "call leaderboard processor"
        )
    }

    if let Some(trust) = args.trust {
        todo!(
            "call trust processor"
        )
    }
}

/// Routes the hackatime specific other commands to their outputs
fn hacka_other_parse(args: CliArgs, widget: bool) {
    if let Some(badge) = args.badge {
        todo!(
            "call badge processor"
        )
    }

    if let Some(lookup) = args.lookup {
        todo!(
            "call lookup processor"
        )
    }
}
