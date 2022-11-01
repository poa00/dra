mod assertions;
mod docker;

use crate::assertions::{assert_contains, assert_error, assert_success};
use crate::docker::{images, users, Docker, ExecArgs};

#[test]
fn installed_successfully() {
    let container = Docker::run(images::UBUNTU);

    let result = container.exec(
        "dra download -s helloworld.deb -i devmatteini/dra-tests",
        ExecArgs::Default,
    );

    let output = assert_success(result);
    assert_contains("Installation completed", &output);
}

#[test]
fn wrong_privileges() {
    let container = Docker::run(images::UBUNTU);

    let result = container.exec(
        "dra download -s helloworld.deb -i devmatteini/dra-tests",
        ExecArgs::User(users::TESTER.into()),
    );

    let output = assert_error(result);
    assert_contains("dpkg", &output);
    assert_contains("requires superuser privilege", &output);
}
