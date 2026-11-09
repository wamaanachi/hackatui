pub mod cli;
pub mod frontend;
pub mod api;
pub mod config;

// so writing a tiny script to test it out 
// maybe change or delete this later

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
// yep we'll def need to change this file later
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use ratatui::widgets::Paragraph;
use frontend::widgets::message::render_message;

// gonna test out all of these now
use frontend::widgets::{
    pie::render_pie,
    progress::render_progress,
    projects::render_fake_ahh_projects,
    today::render_today,
    weeklyGraph::render_weekly,
    rythm::render_rythm,
    project_details::render_project_details,
    project_page::render_project_card,
    calender::render_calender,
    summary::render_summary,
    timeline::render_timeline,
};

// ok so im gonna display them 1 by 1 for now cause i aint making the dashboard :hehehehheheeheheheheheh:
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = config::load_key()?;
    let stats: api::models::Stats = api::fetch_data(&key, "stats/last_7_days").await?;
    let today: api::models::Today = api::fetch_data(&key, "statusbar/today").await?;
    let all_time: api::models::Total = api::fetch_extra(&key, "/api/v1/users/my/stats", &[
        ("total_seconds", "true".into()),
        ("start_date", "1970-01-01T00:00:00Z".into()),
    ]).await?;
    let total = duration(all_time.total_seconds);
    let project_times: Vec<String> = stats.projects.iter()
        .map(|p| duration(p.total_seconds)).collect();
    let projects: Vec<(&str, &str, u16)> = stats.projects.iter()
        .zip(&project_times)
        .map(|(p, time)| {
            let percent = if stats.total_seconds > 0.0 {
                (p.total_seconds / stats.total_seconds * 100.0).clamp(0.0, 100.0) as u16
            } else { 0 };
            (p.name.as_str(), time.as_str(), percent)
        }).collect();
    let languages = slices(&stats.languages);
    let editors = slices(&stats.editors);
    let date = time::OffsetDateTime::now_utc().date();
    let month_start = time::Date::from_calendar_date(date.year(), date.month(), 1)?;
    let monday = date - time::Duration::days(date.weekday().number_days_from_monday() as i64);
    let start = month_start.min(monday - time::Duration::days(21));
    let daily: Vec<api::models::Daily> = api::fetch_data(&key,
        &format!("summaries?start={start}&end={date}&timezone=UTC")).await?;
    let days: Vec<(time::Date, &api::models::Daily)> = daily.iter()
        .map(|day| parse_date(&day.range.date).map(|date| (date, day)))
        .collect::<Result<_, _>>()?;
    let spans: api::models::SpanResponse = api::fetch_extra(&key,
        "/api/v1/users/my/heartbeats/spans", &[
            ("start_date", format!("{monday}T00:00:00Z")),
            ("end_date", format!("{date}T23:59:59.999999Z")),
        ]).await?;
    let project_info: api::models::ProjectResponse = api::fetch_extra(&key,
        &format!("/api/v1/users/{}/projects/details", stats.user_id), &[
            ("since", format!("{start}T00:00:00Z")),
            ("until", format!("{date}T23:59:59.999999Z")),
            ("start", format!("{start}T00:00:00Z")),
            ("end", format!("{date}T23:59:59.999999Z")),
        ]).await?;
    let labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let weekly: Vec<(&str, u64)> = labels.iter().enumerate().map(|(index, label)| {
        let day = monday + time::Duration::days(index as i64);
        (*label, days.iter().filter(|(date, _)| *date == day)
            .map(|(_, data)| data.grand_total.total_seconds).sum::<f64>() as u64 / 60)
    }).collect();
    let rhythm = rhythm_minutes(&spans.spans, monday, date)?;
    let month_activity: Vec<(u32, u64)> = days.iter().filter(|(d, _)| *d >= month_start)
        .map(|(d, data)| (d.day() as u32, data.grand_total.total_seconds as u64 / 60)).collect();
    let calendar_title = format!("{} {} Activity (UTC)", date.month(), date.year());
    let next_month = if date.month() == time::Month::December {
        time::Date::from_calendar_date(date.year() + 1, time::Month::January, 1)?
    } else { time::Date::from_calendar_date(date.year(), date.month().next(), 1)? };
    let month_days = (next_month - month_start).whole_days() as u32;
    let names: Vec<&str> = project_info.projects.iter().map(|p| p.name.as_str()).collect();
    let timeline: Vec<(String, Vec<u64>)> = (0..4).map(|index| {
        let begin = monday - time::Duration::days(21 - index * 7);
        let end = begin + time::Duration::days(7);
        let totals = names.iter().map(|name| days.iter()
            .filter(|(d, _)| *d >= begin && *d < end)
            .flat_map(|(_, day)| &day.projects).filter(|p| p.name == *name)
            .map(|p| p.total_seconds).sum::<f64>() as u64 / 60).collect();
        (begin.to_string(), totals)
    }).collect();
    let mut project_index = 0usize;
    let mut terminal = ratatui::init();

    let seconds = today.grand_total.total_seconds.max(0.0) as u64;
    let minutes = seconds / 60;
    let time = format!("{}h {}m", minutes / 60, minutes % 60);
    let mut selected = '1';

    let result = || -> std::io::Result<()>{
        loop {
            terminal.draw(|frame| {
        // umm yea this code shouldnt be in main lol
                let areas = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Min(0),
                ])
                .split(frame.area());
                
                let menu = Paragraph::new(
                "1 Summary  2 Goal  3 Projects  4 Weekly  5 Languages  6 Editors\n7 Rhythm  8 Details (←/→)  9 Cards  0 Calendar  t Timeline  x Exit",
                );

                frame.render_widget(menu, areas[0]);

                match selected {
                    // '1' => render_message(frame, areas[1], "No Activity", "No coding activity in this period"),

                    '1' => {
                        let rows = Layout::vertical([
                            Constraint::Length(3), Constraint::Min(0),
                        ]).split(areas[1]);
                        render_summary(frame, rows[0], [
                            &total, top(&stats.projects), top(&stats.languages),
                            top(&stats.operating_systems), top(&stats.editors), top(&stats.categories),
                        ]);
                        render_today(frame, rows[1], &time);
                    }
                    '2' => {
                        if let Some(goal) = today.goal.as_ref().filter(|g| g.target_seconds >= 60.0) {
                            render_progress(frame, areas[1], minutes, (goal.target_seconds / 60.0) as u64);
                        } else {
                            render_message(frame, areas[1], "Daily Goal", "No daily goal set in Hackatime");
                        }
                    }
                    '3' => render_fake_ahh_projects(frame, areas[1], &projects),
                    '4' => render_weekly(frame, areas[1], &weekly),
                    '5' => render_pie(frame, areas[1], "Languages (last 7 days)", &languages),
                    '6' => render_pie(frame, areas[1], "Editors (last 7 days)", &editors),

                    '7' => render_rythm(frame, areas[1], &rhythm),
                // i put the example data in the rythm file    

                    '8' => {
                        if let Some(project) = project_info.projects.get(project_index) {
                            let activity: Vec<u64> = (0..=(date - start).whole_days()).map(|i| {
                                let day = start + time::Duration::days(i);
                                days.iter().filter(|(d, _)| *d == day).flat_map(|(_, d)| &d.projects)
                                    .filter(|p| p.name == project.name)
                                    .map(|p| p.total_seconds).sum::<f64>() as u64 / 60
                            }).collect();
                            let week = days.iter().filter(|(d, _)| *d >= monday)
                                .flat_map(|(_, d)| &d.projects).filter(|p| p.name == project.name)
                                .map(|p| p.total_seconds).sum::<f64>();
                            render_project_details(frame, areas[1], &project.name,
                                &duration(project.total_seconds), &duration(week),
                                &project.languages.join(", "), &activity);
                        } else { render_message(frame, areas[1], "Project details", "No projects in this period"); }
                    }
                    '9' => {
                        let visible = project_info.projects.len().min((areas[1].height / 8) as usize);
                        if visible == 0 {
                            render_message(frame, areas[1], "Projects", "No projects or terminal too small");
                        } else {
                            let rows = Layout::vertical(vec![Constraint::Length(8); visible]).split(areas[1]);
                            for (i, project) in project_info.projects.iter().take(visible).enumerate() {
                                render_project_card(frame, rows[i], &project.name,
                                    &duration(project.total_seconds), "", &project.languages.join(", "));
                            }
                        }
                    }
                    '0' => render_calender(frame, areas[1], &month_activity, date.day() as u32,
                        month_days, month_start.weekday().number_days_from_monday() as u32, &calendar_title),
                    't' => render_timeline(frame, areas[1], &timeline, &names),
                    _ => {}
                }
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press{
                    match key.code{
                        KeyCode::Char('x') => break,
                        KeyCode::Left if selected == '8' => { project_index = project_index.saturating_sub(1); }
                        KeyCode::Right if selected == '8' && project_index + 1 < project_info.projects.len() => {
                            project_index += 1;
                        }

                        KeyCode::Char(number @ '1'..='9') => { // lol i coundnt figure out how to loop
                            selected = number;
                        }

                        KeyCode::Char('0') => {
                            selected = '0';
                        }

                        KeyCode::Char('t') => {
                            selected = 't';
                        }
                        _=> {}
                    }
                }
            }
        }
        Ok(())
    }();

    ratatui::restore();
    result?;
    Ok(())
}

fn duration(seconds: f64) -> String {
    let minutes = seconds.max(0.0) as u64 / 60;
    format!("{}h {}m", minutes / 60, minutes % 60)
}

fn top(items: &[api::models::StatItem]) -> &str {
    items.iter().filter(|item| item.total_seconds > 0.0)
        .max_by(|a, b| a.total_seconds.total_cmp(&b.total_seconds))
        .map(|item| item.name.as_str()).unwrap_or("No activity")
}

fn slices(items: &[api::models::StatItem]) -> Vec<(&str, u64, Color)> {
    let colors = [Color::Blue, Color::Yellow, Color::Cyan, Color::Magenta, Color::Green];
    items.iter().enumerate()
        .map(|(i, item)| (item.name.as_str(), item.total_seconds.max(0.0) as u64, colors[i % colors.len()]))
        .collect()
}

fn parse_date(value: &str) -> Result<time::Date, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 3 { return Err("Invalid API date".into()); }
    Ok(time::Date::from_calendar_date(parts[0].parse()?,
        time::Month::try_from(parts[1].parse::<u8>()?)?, parts[2].parse()?)?)
}

fn rhythm_minutes(spans: &[api::models::ActivitySpan], monday: time::Date, today: time::Date)
    -> Result<[[u64; 24]; 7], Box<dyn std::error::Error>> {
    let mut seconds = [[0.0_f64; 24]; 7];
    let start = monday.midnight().assume_utc().unix_timestamp() as f64;
    let end = (today + time::Duration::days(1)).midnight().assume_utc().unix_timestamp() as f64;
    let mut intervals: Vec<(f64, f64)> = spans.iter()
        .filter(|s| s.start_time.is_finite() && s.end_time.is_finite())
        .map(|s| (s.start_time.max(start), s.end_time.min(end)))
        .filter(|(a, b)| a < b).collect();
    intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut covered = start;
    for (begin, finish) in intervals {
        let mut cursor = begin.max(covered);
        while cursor < finish {
            let hour = ((cursor - start) / 3600.0).floor() as usize;
            let stop = finish.min(start + (hour + 1) as f64 * 3600.0);
            seconds[hour / 24][hour % 24] += stop - cursor;
            cursor = stop;
        }
        covered = covered.max(finish);
    }
    Ok(seconds.map(|row| row.map(|s| (s / 60.0).round() as u64)))
}
