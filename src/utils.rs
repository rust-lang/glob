#[inline(always)]
pub(crate) fn get_home_dir() -> Option<String> {
    std::env::home_dir().and_then(|v| v.to_str().map(String::from))
}

// This function is required when `glob_tilde_expansion` field of `glob::MatchOptions` is
// set `true` and pattern starts with `~` followed by any char expect `/`
pub(crate) fn get_user_name() -> Option<String> {
    #[cfg(not(target_os = "windows"))]
    return std::env::var("USER").ok();
    #[cfg(target_os = "windows")]
    std::env::var("USERNAME").ok()
}
