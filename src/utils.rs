#[inline(always)]
pub(crate) fn get_home_dir() -> Option<String> {
    #[allow(deprecated)]
    std::env::home_dir().and_then(|v| v.to_str().map(String::from))
}

// This function is required when `glob_tilde_expansion` field of `glob::MatchOptions` is
// set `true` and pattern starts with `~` followed by any char expect `/`
pub(crate) fn get_user_name() -> Option<String> {
    let varname = if cfg!(windows) { "USERNAME" } else { "USER" };
    std::env::var(varname).ok()
}
