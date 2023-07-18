use dummies::dummy;

#[test]
#[cfg(unix)]
fn test_unix_dummy() {
    assert_eq!(dummy::dummy(), 42)
}

#[test]
#[cfg(windows)]
fn test_windows_dummy() {
    assert_eq!(dummy::dummy(), 21);
}
