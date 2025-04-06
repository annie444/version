use std::time::Duration;

#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/help.toml")
        .case("tests/cmd/major.trycmd")
        .case("tests/cmd/minor.trycmd")
        .case("tests/cmd/patch.trycmd")
        .case("tests/cmd/rc.trycmd")
        .timeout(Duration::new(1, 0))
        .env("RUST_BACKTRACE", "1")
        .run();
}
