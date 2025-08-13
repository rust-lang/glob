#[inline(always)]
pub(crate) fn get_home_dir() -> Option<String> {
    std::env::home_dir().map(|v| v.to_string_lossy().to_string())
}

// This function is required when `glob_tilde_expansion` field of `glob::MatchOptions` is
// set `true` and pattern starts with `~` ollowed by any char expect `/`
pub(crate) fn get_user_name() -> Option<String> {
    if cfg!(target_os = "linux") {
        std::env::var("USER").ok()
    } else {
        None
    }
}
