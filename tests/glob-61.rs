use glob::{glob_with, MatchOptions};
use std::fs;
use std::path::{Path, PathBuf};

fn options(case_sensitive: bool, require_literal_leading_dot: bool) -> MatchOptions {
    MatchOptions {
        case_sensitive,
        require_literal_separator: true,
        require_literal_leading_dot,
    }
}

fn matches(pattern: &Path, options: MatchOptions) -> Vec<PathBuf> {
    let mut paths = glob_with(pattern.to_str().unwrap(), options)
        .unwrap()
        .map(|result| result.unwrap())
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn canonical_set_matches(pattern: &Path, options: MatchOptions, expected: &[PathBuf]) {
    let actual = matches(pattern, options);
    let mut actual = actual
        .iter()
        .map(|path| fs::canonicalize(path).unwrap())
        .collect::<Vec<_>>();
    let mut expected = expected
        .iter()
        .map(|path| fs::canonicalize(path).unwrap())
        .collect::<Vec<_>>();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn insensitive_literals_match_case_variants_and_controls() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().to_path_buf();
    fs::create_dir_all(root.join("MixedParent")).unwrap();
    fs::write(root.join("test_file"), b"lowercase").unwrap();
    fs::write(root.join("MixedParent/test_file"), b"nested").unwrap();
    fs::write(root.join(".hidden"), b"hidden").unwrap();
    let upper = root.join("Test_File");
    let has_distinct_variant = if upper.exists() {
        fs::read(&upper).unwrap() != fs::read(root.join("test_file")).unwrap()
    } else {
        fs::write(&upper, b"uppercase").unwrap();
        true
    };

    let lower = root.join("test_file");
    if has_distinct_variant {
        assert_eq!(
            matches(&lower, options(false, true)),
            vec![root.join("Test_File"), root.join("test_file")]
        );
        assert_eq!(
            matches(&root.join("TEST_FILE"), options(false, true)),
            vec![root.join("Test_File"), root.join("test_file")]
        );
    } else {
        #[cfg(windows)]
        assert_eq!(matches(&lower, options(false, true)), vec![lower.clone()]);
        assert_eq!(
            matches(&root.join("TEST_FILE"), options(false, true)),
            vec![root.join("test_file")]
        );
    }
    assert_eq!(
        matches(&lower, options(true, true)),
        vec![root.join("test_file")]
    );
    assert_eq!(
        matches(&root.join("mixedparent/test_file"), options(false, true)),
        vec![root.join("MixedParent/test_file")]
    );
    assert_eq!(matches(&root.join("*_file"), options(false, true)), {
        let mut expected = vec![root.join("test_file")];
        if has_distinct_variant {
            expected.push(root.join("Test_File"));
        }
        expected.sort();
        expected
    });
    assert_eq!(
        matches(&root.join("missing_file"), options(false, true)),
        Vec::<PathBuf>::new()
    );
    assert_eq!(
        matches(&root.join("test_file/child"), options(false, true)),
        Vec::<PathBuf>::new()
    );
    assert_eq!(
        matches(&root.join(".hidden"), options(false, true)),
        vec![root.join(".hidden")]
    );
    assert_eq!(
        matches(&root.join(".hidden"), options(false, false)),
        vec![root.join(".hidden")]
    );
    assert_eq!(
        matches(&root.join("*hidden"), options(false, true)),
        Vec::<PathBuf>::new()
    );
    assert_eq!(
        matches(&root.join("*hidden"), options(false, false)),
        vec![root.join(".hidden")]
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link = root.join("MixedParent/Broken_Link");
        symlink("missing-target", &link).unwrap();
        assert_eq!(
            matches(&root.join("mixedparent/broken_link"), options(false, true)),
            vec![link]
        );
    }
    let mut physical_variants = vec![lower.clone()];
    if has_distinct_variant {
        physical_variants.push(upper);
    }
    canonical_set_matches(
        &root.join("./test_file"),
        options(false, true),
        &physical_variants,
    );
    canonical_set_matches(
        &root.join("MixedParent/../test_file"),
        options(false, true),
        &physical_variants,
    );
    let repeated_separator = PathBuf::from(format!("{}//test_file", root.display()));
    canonical_set_matches(
        &repeated_separator,
        options(false, true),
        &physical_variants,
    );
    #[cfg(unix)]
    assert_eq!(
        matches(Path::new("/"), options(false, true)),
        vec![PathBuf::from("/")]
    );
}

#[cfg(unix)]
#[test]
fn insensitive_root_literal_returns_root() {
    assert_eq!(
        matches(Path::new("/"), options(false, true)),
        vec![PathBuf::from("/")]
    );
}
