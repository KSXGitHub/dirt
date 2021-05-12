#![cfg(unix)]
use dirt::display_os_string::DisplayOsString;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};

#[test]
fn utf8_os_str() {
    let actual = "abc"
        .pipe(OsString::from)
        .as_os_str()
        .pipe(DisplayOsString)
        .to_string();
    let expected = "abc";
    assert_eq!(actual, expected);
}

#[test]
fn non_utf8_os_str() {
    let actual = vec![0xFF, 0xDD]
        .pipe(OsString::from_vec)
        .as_os_str()
        .pipe(DisplayOsString)
        .to_string();
    let expected = r#""\xFF\xDD""#;
    assert_eq!(actual, expected);
}

#[test]
fn utf8_os_string() {
    let actual = "abc".pipe(OsString::from).pipe(DisplayOsString).to_string();
    let expected = "abc";
    assert_eq!(actual, expected);
}

#[test]
fn non_utf8_os_string() {
    let actual = vec![0xFF, 0xDD]
        .pipe(OsString::from_vec)
        .pipe(DisplayOsString)
        .to_string();
    let expected = r#""\xFF\xDD""#;
    assert_eq!(actual, expected);
}

#[test]
fn utf8_path() {
    let actual = "abc"
        .pipe(PathBuf::from)
        .as_path()
        .pipe(DisplayOsString)
        .to_string();
    let expected = "abc";
    assert_eq!(actual, expected);
}

#[test]
fn non_utf8_path() {
    let actual = vec![0xFF, 0xDD]
        .pipe(OsString::from_vec)
        .pipe(PathBuf::from)
        .as_path()
        .pipe(DisplayOsString)
        .to_string();
    let expected = r#""\xFF\xDD""#;
    assert_eq!(actual, expected);
}

#[test]
fn utf8_path_buf() {
    let actual = "abc".pipe(PathBuf::from).pipe(DisplayOsString).to_string();
    let expected = "abc";
    assert_eq!(actual, expected);
}

#[test]
fn non_utf8_path_buf() {
    let actual = vec![0xFF, 0xDD]
        .pipe(OsString::from_vec)
        .pipe(PathBuf::from)
        .pipe(DisplayOsString)
        .to_string();
    let expected = r#""\xFF\xDD""#;
    assert_eq!(actual, expected);
}

#[test]
fn utf8_os_string_display() {
    let actual = "abc"
        .pipe(OsString::from)
        .pipe(DisplayOsString)
        .pipe(DisplayOsString)
        .pipe(DisplayOsString)
        .to_string();
    let expected = "abc";
    assert_eq!(actual, expected);
}

#[test]
fn non_utf8_os_string_display() {
    let actual = vec![0xFF, 0xDD]
        .pipe(OsString::from_vec)
        .pipe(DisplayOsString)
        .pipe(DisplayOsString)
        .pipe(DisplayOsString)
        .to_string();
    let expected = r#""\xFF\xDD""#;
    assert_eq!(actual, expected);
}
