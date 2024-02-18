use url::Url;

pub(crate) fn is_empty_or_whitespace(s: &str) -> bool {
    s.is_empty() || s.chars().all(char::is_whitespace)
}

pub(crate) fn is_repo_shorthand(s: &str) -> bool {
    if let Some((owner, name)) = s.split_once('/') {
        return owner.chars().all(is_repo_char) && name.chars().all(is_repo_char);
    }

    false
}

pub(crate) fn is_maybe_win_path(s: &str) -> bool {
    let prefix = s.chars().take(2).collect::<Vec<_>>();
    if prefix.len() < 2 {
        return false;
    }

    prefix[0].is_ascii_alphabetic() && prefix[1] == ':'
}

pub(crate) fn to_repo_shorthand(url: &Url) -> Option<String> {
    if let Some(host) = url.host_str() {
        if host == "github.com" {
            if !url.scheme().starts_with("http") {
                return None;
            }
            let p = url.path();
            let Some((author, mut repo)) = p.strip_prefix('/').unwrap_or(p).split_once('/') else {
                return None;
            };
            if author.is_empty() || !author.chars().all(is_repo_char) {
                return None;
            }
            repo = repo.strip_suffix(".git").unwrap_or(repo);
            if repo.is_empty() || !repo.chars().all(is_repo_char) {
                return None;
            }
            return Some(format!("{author}/{repo}"));
        }
    }
    None
}

fn is_repo_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn make_repo_shorthand() {
        let inputs = vec![
            ("https://github.com/foo/bar.git", Some("foo/bar")),
            ("https://github.com/foo/bar", Some("foo/bar")),
            ("ssh://git@github.com/foo/bar", None),
        ];

        for (s, expected) in inputs {
            let url = Url::from_str(s).unwrap();
            let s = to_repo_shorthand(&url);
            assert_eq!(s, expected.map(String::from), "url={url}")
        }
    }
}
