use std::collections::HashMap;

use crate::api::models::Heartbeat;

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
