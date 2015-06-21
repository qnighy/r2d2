use time::Duration;
use r2d2::Config;

#[test]
fn builder() {
    let config = Config::<(), ()>::builder()
        .pool_size(1)
        .helper_threads(2)
        .test_on_check_out(false)
        .initialization_fail_fast(false)
        .connection_timeout(Duration::seconds(3))
        .build();
    assert_eq!(1, config.pool_size());
    assert_eq!(2, config.helper_threads());
    assert_eq!(false, config.test_on_check_out());
    assert_eq!(false, config.initialization_fail_fast());
    assert_eq!(Duration::seconds(3), config.connection_timeout());
}

#[test]
#[should_panic(expected = "pool_size must be positive")]
fn builder_zero_pool_size() {
    Config::<(), ()>::builder().pool_size(0);
}

#[test]
#[should_panic(expected = "helper_threads must be positive")]
fn builder_zero_helper_threads() {
    Config::<(), ()>::builder().helper_threads(0);
}

#[test]
#[should_panic(expected = "connection_timeout must be positive")]
fn builder_zero_connection_timeout() {
    Config::<(), ()>::builder().connection_timeout(Duration::zero());
}

#[test]
#[should_panic(expected = "connection_timeout must be positive")]
fn builder_negative_connection_timeout() {
    Config::<(), ()>::builder().connection_timeout(Duration::seconds(-1));
}
