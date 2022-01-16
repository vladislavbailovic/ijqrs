use std::collections::HashMap;

const USAGE: &str = "Usage";
const SHORTCUTS: &str = "Shortcuts";
const BOOKMARKS: &str = "Bookmarks";
const COMMANDS: &str = "Commands";

// TODO: preserve ordering somehow
pub fn get_help() -> HashMap<String, Vec<String>> {
    HashMap::from([
        (
            USAGE.to_string(),
            vec![
                "ijqrd [FILE_NAME]".to_string(),
                "If file name is ommited, will read from STDIN".to_string(),
            ],
        ),
        (
            SHORTCUTS.to_string(),
            vec![
                "\t - Ctrl+q: Quit".to_string(),
                "\t - Ctrl+w: Switch panel".to_string(),
                "\t - Esc: Switch mode".to_string(),
                "\t - /: Start pattern search (in content panels)".to_string(),
                "\t - Enter: Apply search pattern (in content panels search mode)".to_string(),
                "\t - n: Next match (in content panels search mode)".to_string(),
                "\t - N: Previous match (in content panels search mode)".to_string(),
                "\t - Ctrl+l: Clear search (in content panels search mode)".to_string(),
                "\t - ?: Show help".to_string(),
            ],
        ),
        (
            BOOKMARKS.to_string(),
            vec![
                "\t - Ctrl+s: Open/Close bookmarks interface".to_string(),
                "\t - Ctrl+a: Add bookmark".to_string(),
                "\t - Ctrl+d|Del: Delete bookmark".to_string(),
                "\t - Enter: Use bookmark".to_string(),
            ],
        ),
        (
            COMMANDS.to_string(),
            vec![
                "\t - :wc [FILE_NAME]: write the jq command string to file".to_string(),
                "\t - :w [FILE_NAME]: write the command output to file".to_string(),
                "\t - :r [FILE_NAME]: (re)-run the jq command string".to_string(),
            ],
        ),
    ])
}
