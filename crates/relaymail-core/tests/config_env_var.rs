use relaymail_core::config::{
    parse_csv_list, parse_duration_seconds, read_bool, read_optional, read_required, read_u32,
    read_u64, ConfigError,
};

struct EnvGuard(&'static str);
impl Drop for EnvGuard {
    fn drop(&mut self) {
        std::env::remove_var(self.0);
    }
}
fn set(name: &'static str, value: &str) -> EnvGuard {
    std::env::set_var(name, value);
    EnvGuard(name)
}

#[test]
fn required_and_optional() {
    let _g = set("RM_TEST_REQ_A", "hello");
    assert_eq!(read_required("RM_TEST_REQ_A").unwrap(), "hello");
    assert_eq!(read_optional("RM_TEST_REQ_A"), Some("hello".to_string()));
    assert!(read_optional("RM_TEST_MISSING_KEY_XYZ").is_none());
    let err = read_required("RM_TEST_MISSING_KEY_XYZ").unwrap_err();
    assert!(matches!(err, ConfigError::MissingVar(_)));
}

#[test]
fn bools_and_numbers() {
    let _a = set("RM_TEST_BOOL_A", "true");
    let _b = set("RM_TEST_BOOL_B", "NO");
    assert!(read_bool("RM_TEST_BOOL_A", false).unwrap());
    assert!(!read_bool("RM_TEST_BOOL_B", true).unwrap());
    assert!(!read_bool("RM_TEST_BOOL_DEFAULT", false).unwrap());

    let _u = set("RM_TEST_U32", "42");
    let _l = set("RM_TEST_U64", "9999999999");
    assert_eq!(read_u32("RM_TEST_U32", 0).unwrap(), 42);
    assert_eq!(read_u64("RM_TEST_U64", 0).unwrap(), 9_999_999_999);
    let _bad = set("RM_TEST_U32_BAD", "abc");
    assert!(read_u32("RM_TEST_U32_BAD", 0).is_err());
}

#[test]
fn duration_and_list() {
    assert_eq!(
        parse_duration_seconds("t", "45s").unwrap(),
        std::time::Duration::from_secs(45)
    );
    assert!(parse_duration_seconds("t", "garbage").is_err());
    assert_eq!(parse_csv_list("a, b,c ,,").len(), 3);
}
