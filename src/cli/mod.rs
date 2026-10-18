// this module is for the cli stuff

pub mod cli;

use cli::CliArgs;

pub(crate) fn parse(args: CliArgs) {
    todo!(
        "Add parser"
    )
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

