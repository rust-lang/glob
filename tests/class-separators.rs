use glob::{glob, glob_with, MatchOptions, Pattern};
use std::fs;
use std::path::PathBuf;

#[test]
fn character_classes_with_separators() {
    let root = tempfile::tempdir().unwrap();
    let base = root.path().join("λ");
    fs::create_dir_all(base.join("a/b/sub")).unwrap();
    fs::create_dir_all(base.join("a/^")).unwrap();
    for file in [
        "a/c",
        "a/d",
        "a/]",
        "a/[",
        "a/α",
        "a/b/leaf",
        "a/b/sub/leaf",
        "a/^/]",
    ] {
        fs::write(base.join(file), b"preserved contents").unwrap();
    }
    let cases: &[(&str, &[&str])] = &[
        ("a/[b/c]", &["a/b", "a/c"]),
        ("a/[b/c]/", &["a/b"]),
        ("a/[b/c]/leaf", &["a/b/leaf"]),
        ("a/[b/c]/**/leaf", &["a/b/leaf", "a/b/sub/leaf"]),
        ("a/[!b/]", &["a/[", "a/]", "a/^", "a/c", "a/d", "a/α"]),
        ("a/[]/]", &["a/]"]),
        ("a/[!]/]", &["a/[", "a/^", "a/b", "a/c", "a/d", "a/α"]),
        ("a/[[/]", &["a/["]),
        ("a/[α/]", &["a/α"]),
        ("a/[b-c/]", &["a/b", "a/c"]),
        ("a/[b]", &["a/b"]),
        ("a/[b]/leaf", &["a/b/leaf"]),
        ("a/[]]", &["a/]"]),
        ("a/[[]", &["a/["]),
        ("a/[^]/]", &["a/^/]"]),
    ];
    let mut failures = Vec::new();
    for (pattern, expected) in cases {
        let pattern = format!("{}/{}", base.display(), pattern);
        let mut expected: Vec<PathBuf> = expected.iter().map(|p| base.join(p)).collect();
        expected.sort();
        for options in [None, Some(MatchOptions::new())] {
            let paths = match options {
                None => glob(&pattern),
                Some(options) => glob_with(&pattern, options),
            };
            let paths = match paths {
                Ok(paths) => paths,
                Err(error) => {
                    failures.push(format!("{pattern}: {error}"));
                    continue;
                }
            };
            let mut found: Vec<_> = paths.map(|p| p.unwrap()).collect();
            found.sort();
            if found != expected {
                failures.push(format!("{pattern}: {found:?} != {expected:?}"));
            }
        }
    }
    let invalid = format!("{}/a/[b/c", base.display());
    assert_eq!(
        glob(&invalid).unwrap_err().to_string(),
        Pattern::new(&invalid).unwrap_err().to_string()
    );
    assert!(
        failures.is_empty(),
        "{} failures: {failures:?}",
        failures.len()
    );
}
