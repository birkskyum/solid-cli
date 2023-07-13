use std::path::PathBuf;

use serde_json::json;
use swc_core::common::{chain, Mark};
use swc_core::{
    ecma::parser::{EsConfig, Syntax},
    ecma::transforms::base::resolver,
    ecma::transforms::testing::test_fixture,
    ecma::visit::as_folder,
};

use testing::fixture;
use transform_config::config::{Config, Testing};
use transform_config::TransformVisitor;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/basic_tests/**/code.js")]
fn jsx_dom_expressions_fixture_babel(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        &|_t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(TransformVisitor::new(Config::test()))
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
#[fixture("tests/fixture/merge_tests/**/code.js")]
fn merge_tests(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        &|_t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(TransformVisitor::new(Config {
                    additional_plugins: vec![(
                        "UnoCss".to_string(),
                        "unocss/vite".to_string(),
                        true,
                        json!({"flag2": false}),
                    )],
                    force_transform: true,
                    merge_configs: true,
                }))
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
