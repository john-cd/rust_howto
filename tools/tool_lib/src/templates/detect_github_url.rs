// TODO Reviewed ??-??-2026
/// Detect a GitHub URL, returning `(detected, username, repo)`, where `detected` is `true` if it is a proper Github URL,
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
    fn test_detect_github_url_valid() {
        let valid_cases = [
            ("https://github.com/user/repo", "user", "repo"),
            ("http://github.com/another-user/another-repo.git", "another-user", "another-repo"),
            ("https://www.github.com/user/repo", "user", "repo"),
            (
                "https://github.com/user_with_underscores/repo-with-dashes/",
                "user_with_underscores",
                "repo-with-dashes",
            ),
            ("https://github.com/user.name/repo.js", "user.name", "repo.js"),
            ("https://github.com/user/repo/tree/main", "user", "repo"),
            ("https://github.com/user/repo/issues/123", "user", "repo"),
            ("https://github.com/user/repo/blob/main/README.md", "user", "repo"),
        ];

        for (url, expected_user, expected_repo) in valid_cases {
            assert_eq!(
                detect_github_url(url),
                (true, expected_user, expected_repo),
                "Failed for valid URL: {url}"
            );
        }
    }

    #[test]
    fn test_detect_github_url_invalid() {
        let invalid_cases = [
            "",
            "   ",
            "\t\n",
            "not a url",
            "github.com/user/repo",
            "https://github.com",
            "https://github.com/",
            "https://github.com/user",
            "https://github.com/user/",
            "https://github.com//repo",
            "https://gitlab.com/user/repo",
            "https://notgithub.com/user/repo",
            "https://github.com.attacker.com/user/repo",
            "ftp://github.com/user/repo",
            "git@github.com:user/repo.git",
            "ssh://git@github.com/user/repo",
        ];

        for url in invalid_cases {
            assert_eq!(
                detect_github_url(url),
                (false, "", ""),
                "Failed for invalid URL: {url}"
            );
        }
    }
}
