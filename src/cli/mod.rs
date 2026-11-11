// this module is for the cli stuff

pub mod cli;

use ratatui::backend::CrosstermBackend;
use ratatui::{Terminal, TerminalOptions, Viewport};
use std::io::stdout;

use crate::api::models::DashboardSummary;
use crate::cli::cli::CliArgs;
use crate::frontend::widgets::{
    project_details::render_project_details, projects::render_fake_ahh_projects,
    summary::render_summary, today::render_today,
};

/// Renders a single Ratatui widget inline in the terminal scrollback and exits cleanly.
fn render_inline<F>(height: u16, render_fn: F) -> std::io::Result<()>
where
    F: FnOnce(&mut ratatui::Frame, ratatui::layout::Rect),
{
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(height),
        },
    )?;

    terminal.draw(|frame| {
        render_fn(frame, frame.area());
    })?;

    // Blank line so your shell prompt starts cleanly under the widget
    println!();
    Ok(())
}

/// Routes the cli args to their various outputs
pub(crate) fn parse(args: CliArgs, summary: &DashboardSummary) {
    let widget = !args.no_widget;

    waka_parse(args.clone(), widget, summary);
    hacka_parse(args.clone(), widget, summary);
    hacka_global_parse(args.clone(), widget);
    hacka_other_parse(args.clone(), widget);
}

/// Routes the wakatime specific commands to their outputs
fn waka_parse(args: CliArgs, widget: bool, summary: &DashboardSummary) {
    if args.current_user {
        println!("Authenticated with HackaTime API key.");
    }

    if args.summary || args.waka_stats {
        if widget {
            let _ = render_inline(5, |frame, area| {
                render_summary(frame, area, summary);
            });
        } else {
            print_cli_summary(summary);
        }
    }

    if args.status {
        if widget {
            let _ = render_inline(3, |frame, area| {
                render_today(frame, area, summary);
            });
        } else {
            let secs = summary.total_seconds as u64;
            println!(
                "Today's Time: {}h {}m | Streak: {} days",
                secs / 3600,
                (secs % 3600) / 60,
                summary.streak
            );
        }
    }
}

/// Routes the hackatime specific main commands to their outputs
fn hacka_parse(args: CliArgs, widget: bool, summary: &DashboardSummary) {
    if args.stats {
        if widget {
            let height = (summary.projects.len() as u16 + 2).clamp(4, 10);
            let _ = render_inline(height, |frame, area| {
                render_fake_ahh_projects(frame, area, &summary.projects);
            });
        } else {
            print_cli_summary(summary);
        }
    }

    if let Some(heartbeat) = args.heartbeat {
        println!("Heartbeat details for: {heartbeat} (coming soon)");
    }

    if let Some(project) = args.project_details {
        if widget {
            let _ = render_inline(9, |frame, area| {
                render_project_details(frame, area, summary);
            });
        } else if let Some(p) = summary.projects.iter().find(|p| p.name == project) {
            let secs = p.total_seconds as u64;
            println!("Project: {}", p.name);
            println!(
                "Time:    {}h {}m ({:.1}%)",
                secs / 3600,
                (secs % 3600) / 60,
                p.percent
            );
        } else {
            println!("Project '{project}' not found in recent activity.");
        }
    }
}

/// Routes the hackatime specific global userbase commands to their outputs
fn hacka_global_parse(args: CliArgs, _widget: bool) {
    if args.currently_hacking {
        println!("Currently hacking status not yet implemented.");
    }

    if args.banned {
        println!("Banned users check not yet implemented.");
    }

    if let Some(leaderboard) = args.leaderboard {
        println!("Leaderboard for '{leaderboard}' not yet implemented.");
    }

    if let Some(trust) = args.trust {
        println!("Trust value for '{trust}' not yet implemented.");
    }
}

/// Routes the hackatime specific other commands to their outputs
fn hacka_other_parse(args: CliArgs, _widget: bool) {
    if let Some(badge) = args.badge {
        println!("Badge URL for '{badge}' not yet implemented.");
    }

    if let Some(lookup) = args.lookup {
        println!("Lookup for '{lookup}' not yet implemented.");
    }
}

fn print_cli_summary(summary: &DashboardSummary) {
    let secs = summary.total_seconds as u64;
    println!("=== HackaTime Summary ===");
    println!("Total Time: {}h {}m", secs / 3600, (secs % 3600) / 60);
    println!("Streak:     {} days\n", summary.streak);

    if !summary.projects.is_empty() {
        println!("Projects:");
        for p in &summary.projects {
            let p_secs = p.total_seconds as u64;
            println!(
                "  {:<16} {:>2}h {:02}m  ({:.1}%)",
                p.name,
                p_secs / 3600,
                (p_secs % 3600) / 60,
                p.percent
            );
        }
        println!();
    }

    if !summary.languages.is_empty() {
        println!("Languages:");
        for l in &summary.languages {
            let l_secs = l.total_seconds as u64;
            println!(
                "  {:<16} {:>2}h {:02}m  ({:.1}%)",
                l.name,
                l_secs / 3600,
                (l_secs % 3600) / 60,
                l.percent
            );
        }
        println!();
    }

    if !summary.editors.is_empty() {
        println!("Editors:");
        for e in &summary.editors {
            let e_secs = e.total_seconds as u64;
            println!(
                "  {:<16} {:>2}h {:02}m  ({:.1}%)",
                e.name,
                e_secs / 3600,
                (e_secs % 3600) / 60,
                e.percent
            );
        }
    }
}
