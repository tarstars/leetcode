use super::*;

use std::path::{Component, Path};

/// Resolves the path using the standard library's own parser for the tokenizing
/// half. `Path::components` already collapses repeated slashes, drops `.`,
/// keeps `..` as a `ParentDir`, and treats `...` as an ordinary name — exactly
/// this problem's rules — so the only logic left here is popping on `..`. That
/// makes it independent of a hand-rolled `split('/')`.
fn reference(path: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();

    for component in Path::new(path).components() {
        match component {
            Component::Normal(name) => stack.push(name.to_str().expect("ASCII input")),
            Component::ParentDir => {
                stack.pop();
            }
            Component::RootDir | Component::CurDir | Component::Prefix(_) => {}
        }
    }

    format!("/{}", stack.join("/"))
}

fn simplify(path: &str) -> String {
    Solution::simplify_path(path.to_owned())
}

/// Checks the output is in canonical form, independently of what it resolves
/// to: leading slash, no doubled slashes, no trailing slash except at the root,
/// and no `.` or `..` left anywhere.
fn assert_canonical(input: &str, output: &str) {
    assert!(
        output.starts_with('/'),
        "{input:?} -> {output:?} does not start with a slash"
    );
    assert!(
        !output.contains("//"),
        "{input:?} -> {output:?} has a doubled slash"
    );
    assert!(
        output == "/" || !output.ends_with('/'),
        "{input:?} -> {output:?} ends with a slash"
    );

    for part in output.split('/').skip(1) {
        if output == "/" {
            continue;
        }
        assert!(
            !part.is_empty(),
            "{input:?} -> {output:?} has an empty part"
        );
        assert!(part != ".", "{input:?} -> {output:?} still holds a '.'");
        assert!(part != "..", "{input:?} -> {output:?} still holds a '..'");
    }
}

fn check(path: &str) {
    let got = simplify(path);
    assert_canonical(path, &got);
    assert_eq!(got, reference(path), "for {path:?}");
}

/// Deterministic path generation, so any failure reproduces.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next_value() % bound as u64) as usize
    }

    /// Builds a path out of the pieces that matter: ordinary names, `.`, `..`,
    /// runs of periods that are really names, and extra slashes.
    fn path(&mut self, parts: usize) -> String {
        const NAMES: [&str; 8] = ["a", "b", "home", "user", "...", "....", "x_1", "9"];
        let mut path = String::from("/");

        for _ in 0..parts {
            match self.below(10) {
                0 | 1 => path.push_str(".."),
                2 => path.push('.'),
                _ => path.push_str(NAMES[self.below(NAMES.len())]),
            }
            for _ in 0..=self.below(3) {
                path.push('/');
            }
        }

        path
    }
}

#[test]
fn example_1() {
    assert_eq!(simplify("/home/"), "/home");
}

#[test]
fn example_2() {
    assert_eq!(simplify("/home//foo/"), "/home/foo");
}

#[test]
fn example_3() {
    assert_eq!(
        simplify("/home/user/Documents/../Pictures"),
        "/home/user/Pictures"
    );
}

/// Going up from the root stays at the root.
#[test]
fn example_4() {
    assert_eq!(simplify("/../"), "/");
}

/// "..." is an ordinary directory name, not a parent reference.
#[test]
fn example_5() {
    assert_eq!(simplify("/.../a/../b/c/../d/./"), "/.../b/d");
}

/// The root itself, and every way of writing it.
#[test]
fn the_root() {
    assert_eq!(simplify("/"), "/");
    assert_eq!(simplify("//"), "/");
    assert_eq!(simplify("///"), "/");
    assert_eq!(simplify("/."), "/");
    assert_eq!(simplify("/./"), "/");
    assert_eq!(simplify("/.."), "/");
    assert_eq!(simplify("/../.."), "/");
    assert_eq!(simplify("/a/.."), "/");
    assert_eq!(simplify("/a/b/../.."), "/");
}

/// A run of three or more periods is a name, and so is a name that merely
/// starts with periods.
#[test]
fn runs_of_periods_are_names() {
    assert_eq!(simplify("/..."), "/...");
    assert_eq!(simplify("/...."), "/....");
    assert_eq!(simplify("/....."), "/.....");
    assert_eq!(simplify("/a/.../b"), "/a/.../b");
    assert_eq!(simplify("/.../..."), "/.../...");
    assert_eq!(simplify("/.../.."), "/");
    assert_eq!(simplify("/../..."), "/...");
}

/// A single period is dropped wherever it appears.
#[test]
fn single_periods_are_dropped() {
    assert_eq!(simplify("/a/./b"), "/a/b");
    assert_eq!(simplify("/./a"), "/a");
    assert_eq!(simplify("/a/."), "/a");
    assert_eq!(simplify("/./././"), "/");
}

/// Runs of slashes collapse, wherever they appear.
#[test]
fn runs_of_slashes_collapse() {
    assert_eq!(simplify("//home"), "/home");
    assert_eq!(simplify("/home///foo"), "/home/foo");
    assert_eq!(simplify("////a////b////"), "/a/b");
}

/// A parent reference can never climb above the root, however many there are.
#[test]
fn parents_never_escape_the_root() {
    assert_eq!(simplify("/../../../../"), "/");
    assert_eq!(simplify("/a/../../../b"), "/b");
    assert_eq!(simplify("/a/b/c/../../../../../d"), "/d");
}

/// Names may hold digits and underscores.
#[test]
fn names_with_digits_and_underscores() {
    assert_eq!(simplify("/a_1/b_2/"), "/a_1/b_2");
    assert_eq!(simplify("/9/0/../8"), "/9/8");
    assert_eq!(simplify("/_/__/___"), "/_/__/___");
}

/// The reference must reproduce the statement's examples before it is trusted
/// as an oracle for the generated paths.
#[test]
fn the_reference_agrees_with_the_statement() {
    assert_eq!(reference("/home/"), "/home");
    assert_eq!(reference("/home//foo/"), "/home/foo");
    assert_eq!(
        reference("/home/user/Documents/../Pictures"),
        "/home/user/Pictures"
    );
    assert_eq!(reference("/../"), "/");
    assert_eq!(reference("/.../a/../b/c/../d/./"), "/.../b/d");
}

/// Generated paths mixing names, dots, double dots and extra slashes.
#[test]
fn matches_reference_on_generated_paths() {
    let mut rng = Rng(0x1234_5678_9ABC_DEF0);

    for parts in 1..=40 {
        for _ in 0..50 {
            let path = rng.path(parts);
            check(&path);
        }
    }
}

/// Simplifying an already simplified path must change nothing.
#[test]
fn simplifying_is_idempotent() {
    let mut rng = Rng(0x0FED_CBA9_8765_4321);

    for parts in 1..=30 {
        for _ in 0..30 {
            let once = simplify(&rng.path(parts));
            assert_eq!(simplify(&once), once, "not idempotent for {once:?}");
        }
    }
}

/// 3000 characters is the constraint's maximum, reached three ways: a deep
/// path, a long run of slashes, and a long run of parent references.
#[test]
fn longest_allowed_input() {
    let deep: String = std::iter::repeat("/ab").take(1000).collect();
    assert_eq!(deep.len(), 3000);
    check(&deep);

    let slashes = "/".repeat(3000);
    assert_eq!(simplify(&slashes), "/");

    let climb = format!("/a{}", "/..".repeat(999));
    assert!(climb.len() <= 3000);
    assert_eq!(simplify(&climb), "/");

    let alternating: String = std::iter::repeat("/a/..").take(600).collect();
    assert_eq!(simplify(&alternating), "/");

    let dots: String = std::iter::repeat("/...").take(750).collect();
    assert_eq!(dots.len(), 3000);
    check(&dots);
}
