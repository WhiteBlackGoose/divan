// Tests that entry benchmarks/groups have correct generated properties.

// Miri does not work with `linkme`.
#![cfg(not(miri))]

use divan::__private::{EntryMeta, BENCH_ENTRIES, GROUP_ENTRIES};

#[divan::bench]
fn outer() {}

#[divan::bench_group]
mod outer_group {
    #[divan::bench]
    fn inner() {}

    #[divan::bench_group]
    mod inner_group {}
}

#[divan::bench]
#[ignore]
fn ignored() {}

#[divan::bench_group]
#[allow(unused_attributes)]
#[ignore]
mod ignored_group {
    #[divan::bench]
    fn not_yet_ignored() {}
}

/// Finds `EntryMeta` based on the entry's raw name.
macro_rules! find_meta {
    ($entries:expr, $raw_name:literal) => {
        $entries
            .iter()
            .map(|entry| &entry.meta)
            .find(|common| common.raw_name == $raw_name)
            .expect(concat!($raw_name, " not found"))
    };
}

fn find_outer() -> &'static EntryMeta {
    find_meta!(BENCH_ENTRIES, "outer")
}

fn find_inner() -> &'static EntryMeta {
    find_meta!(BENCH_ENTRIES, "inner")
}

fn find_outer_group() -> &'static EntryMeta {
    find_meta!(GROUP_ENTRIES, "outer_group")
}

fn find_inner_group() -> &'static EntryMeta {
    find_meta!(GROUP_ENTRIES, "inner_group")
}

#[test]
fn file() {
    let file = file!();

    assert_eq!(find_outer().location.file, file);
    assert_eq!(find_outer_group().location.file, file);

    assert_eq!(find_inner().location.file, file);
    assert_eq!(find_inner_group().location.file, file);
}

#[test]
fn module_path() {
    let outer_path = module_path!();
    assert_eq!(find_outer().module_path, outer_path);
    assert_eq!(find_outer_group().module_path, outer_path);

    let inner_path = format!("{outer_path}::outer_group");
    assert_eq!(find_inner().module_path, inner_path);
    assert_eq!(find_inner_group().module_path, inner_path);
}

#[test]
fn line() {
    assert_eq!(find_outer().location.line, 8);
    assert_eq!(find_outer_group().location.line, 11);

    assert_eq!(find_inner().location.line, 13);
    assert_eq!(find_inner_group().location.line, 16);
}

#[test]
fn column() {
    assert_eq!(find_outer().location.col, 1);
    assert_eq!(find_outer_group().location.col, 1);

    assert_eq!(find_inner().location.col, 5);
    assert_eq!(find_inner_group().location.col, 5);
}

#[test]
fn ignore() {
    assert!(find_meta!(BENCH_ENTRIES, "ignored").ignore);
    assert!(find_meta!(GROUP_ENTRIES, "ignored_group").ignore);

    // Although its parent is marked as `#[ignore]`, it itself is not yet known
    // to be ignored.
    assert!(!find_meta!(BENCH_ENTRIES, "not_yet_ignored").ignore);

    assert!(!find_inner().ignore);
    assert!(!find_inner_group().ignore);
    assert!(!find_outer().ignore);
    assert!(!find_outer_group().ignore);
}