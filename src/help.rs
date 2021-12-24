use super::events;

pub fn usage() -> String {
    String::from("Usage: \n")
        + "ijqrs [FILE_NAME]\n"
        + "\n"
        + "If file name is ommited, will read from STDIN\n"
}

pub fn shortcuts() -> String {
    let mut res: Vec<String> = vec![String::from("Global keys")];
    for (key, help) in events::global_keys().iter() {
        res.push(format!("    <{}>: {}", key, help));
    }
    return res.join("\n");
}
