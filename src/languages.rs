use regex::Regex;
use std::collections::HashSet;

use crate::markdown::get_client;

const LANGUAGES_YML_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/languages.yml";

const EXTENSION_PATTERN: &str = r#"(?m)^\s*extensions:\s+-\s+"\.([\w\d]+)""#;

// Find all lines that look like `- ".ext"` and extract the extension. Each extension should be added to the set returned.
pub fn get_languages() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let client = get_client();
    let mut languages = HashSet::new();
    let languages_yml = client.get(LANGUAGES_YML_URL).send()?.text()?;
    let pattern = Regex::new(EXTENSION_PATTERN)?;

    pattern
        .captures_iter(languages_yml.as_str())
        .for_each(|captures| {
            let extension = captures.get(1).unwrap().as_str();
            languages.insert(extension.to_string());
        });

    Ok(languages)
}
