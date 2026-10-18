use std::collections::HashMap;

use crate::api::models::Heartbeat;

const MAX_GAP_SECS: f64 = 120.0;

pub fn time_by<F>(heartbeats: &[Heartbeat], field: F) -> (Vec<(String, f64)>, f64)
where
    F: Fn(&Heartbeat) -> Option<&str>,
{
    let mut sorted: Vec<&Heartbeat> = heartbeats.iter().collect();
    sorted.sort_by(|a, b| a.time.total_cmp(&b.time));

    let mut totals: HashMap<String, f64> = HashMap::new();
    let mut missing = 0.0;

    for pair in sorted.windows(2) {
        let (cur, next) = (pair[0], pair[1]);
        let gap = (next.time - cur.time).min(MAX_GAP_SECS);
        match field(cur) {
            Some(name) => *totals.entry(name.to_string()).or_insert(0.0) += gap,
            None => missing += gap,
        }
    }

    let mut out: Vec<(String, f64)> = totals.into_iter().collect();
    out.sort_by(|a, b| b.1.total_cmp(&a.1));
    (out, missing)
}

///count_by lets us get a count of a field from a vector of heartbeats
///It accepts 2 args, one is the vector of raw heartbeats, other is a closure which captures a single
///heartbeat and returns an Option string slice. This string slice is supposed to be a name from
///field, like operating_system or editor, language, etc.
pub fn count_by<F>(heartbeats: &[Heartbeat], field: F) -> (Vec<(String, u64)>, u64)
where
    F: Fn(&Heartbeat) -> Option<&str>,
{
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut missing = 0;

    for hb in heartbeats {
        match field(hb) {
            Some(name) => *counts.entry(name.to_string()).or_insert(0) += 1,
            None => missing += 1,
        }
    }

    let mut sorted: Vec<(String, u64)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    (sorted, missing)
}

#[cfg(test)]
mod test {
    use super::*;

    fn hb(editor: Option<&str>) -> Heartbeat {
        Heartbeat {
            id: 0,
            entity: None,
            entity_type: None,
            category: None,
            time: 0.0,
            project: None,
            branch: None,
            language: None,
            is_write: None,
            editor: editor.map(String::from),
            operating_system: None,
            machine: None,
            cursorpos: None,
            lineno: None,
            lines: None,
            line_additions: None,
            line_deletions: None,
        }
    }

    fn hb_at(time: f64, editor: Option<&str>) -> Heartbeat {
        let mut h = hb(editor);
        h.time = time;
        h
    }

    #[test]
    fn attributes_capped_gaps_to_editor() {
        let heartbeats = vec![
            hb_at(0.0, Some("neovim")),
            hb_at(60.0, Some("neovim")),
            hb_at(120.0, Some("vscode")),
            hb_at(600.0, Some("vscode")),
        ];

        let (shares, _) = time_by(&heartbeats, |h| h.editor.as_deref());
        let find = |name: &str| shares.iter().find(|(n, _)| n == name).map(|(_, t)| *t);

        assert_eq!(find("neovim"), Some(120.0));
        assert_eq!(find("vscode"), Some(120.0));
    }

    #[test]
    fn counts_editors_and_missing() {
        let heartbeats = vec![
            hb(Some("neovim")),
            hb(Some("neovim")),
            hb(Some("vscode")),
            hb(None),
        ];

        let (counts, missing) = count_by(&heartbeats, |h| h.editor.as_deref());
        assert_eq!(counts[0], ("neovim".to_string(), 2));
        assert_eq!(counts[1], ("vscode".to_string(), 1));
        assert_eq!(missing, 1);
    }
}
