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

