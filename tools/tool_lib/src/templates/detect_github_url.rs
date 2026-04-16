/// Detect a GitHub URL, returning `(detected, username, repo)`, where `detected`` is `true` if it is a proper Github URL,
/// `false` otherwise. In the latter case, `username` and `repo` are empty string slices.
pub fn detect_github_url(url: &str) -> (bool, &str, &str) {
    let re2: &regex::Regex = crate::regex!(
        r"^http(?:s)?://(?:www\.)?github.com/(?<username>[\w_\.\-]+)/(?<repo>[\w_\.\-]+?)(?:/.*|\.git)?$"
    );
    if let Some(caps) = re2.captures(url) {
        return (
            true,
            caps.name("username").unwrap().as_str(),
            caps.name("repo").unwrap().as_str(),
        );
    }
    (false, "", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_github_url() {
        assert_eq!(
            detect_github_url("https://github.com/user/repo"),
            (true, "user", "repo"),
            "https://github.com/user/repo"
        );
        assert_eq!(
            detect_github_url("http://github.com/another-user/another-repo.git"),
            (true, "another-user", "another-repo"),
            "http://github.com/another-user/another-repo.git"
        );
        assert_eq!(
            detect_github_url("https://github.com/user_with_underscores/repo-with-dashes/"),
            (true, "user_with_underscores", "repo-with-dashes"),
            "https://github.com/user_with_underscores/repo-with-dashes/"
        );
        assert_eq!(
            detect_github_url("https://github.com/user/repo/tree/main"),
            (true, "user", "repo"),
            "https://github.com/user/repo/tree/main"
        );
        assert_eq!(
            detect_github_url("https://github.com/user/"),
            (false, "", ""),
            "https://github.com/user/"
        );
        assert_eq!(
            detect_github_url("https://github.com/user"),
            (false, "", ""),
            "https://github.com/user"
        );
        assert_eq!(detect_github_url("https://github.com/"), (false, "", ""));
        assert_eq!(
            detect_github_url("https://gitlab.com/user/repo"),
            (false, "", "")
        );
        assert_eq!(detect_github_url("not a url"), (false, "", ""));
        assert_eq!(detect_github_url(""), (false, "", ""));
    }
}
