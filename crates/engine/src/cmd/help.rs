//! Help → the project's links: VansonCraft's source repository on GitHub (its issues and releases
//! are there too). The commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

/// The app's id: its repository's and its executables' name.
pub const APP_ID: &str = "vansoncraft";
/// The GitHub repository VansonCraft is developed in (`owner/name`).
pub const REPOSITORY: &str = "vansyson1308/vansoncraft";

/// This app's source repository.
pub fn github_url() -> String {
    format!("https://github.com/{REPOSITORY}")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.github", "VansonCraft on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {github}", always, |_, _| Ok(json!({ "github": github_url() }))),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/vansyson1308/vansoncraft");
        assert_eq!(s.execute("help.links", &json!({})).unwrap(), json!({"github": "https://github.com/vansyson1308/vansoncraft"}));
        for gone in ["help.discord", "help.website", "help.appPage"] {
            assert!(s.execute(gone, &json!({})).is_err(), "{gone}: the upstream project's links are not this app's");
        }
    }
}
