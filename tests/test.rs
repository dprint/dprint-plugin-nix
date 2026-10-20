// SPDX-License-Identifier: MPL-2.0

extern crate dprint_development;
extern crate dprint_plugin_nix;

use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_nix::configuration::Configuration;
use dprint_plugin_nix::configuration::resolve_config;
use dprint_plugin_nix::*;

#[test]
fn test_specs() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.nix",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      Arc::new(move |file_path, file_text, _range, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve_config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        format_text(file_path, file_text, &config_result.config).map_err(|err| err.into())
      })
    },
    Arc::new(move |_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  )
}

#[test]
fn should_have_schema_defaults_matching_resolved_config() {
  let schema: serde_json::Value = serde_json::from_str(include_str!("../deployment/schema.json")).unwrap();
  let properties = schema["properties"].as_object().unwrap();
  let config = serde_json::to_value(default_config()).unwrap();
  let config = config.as_object().unwrap();
  assert_eq!(properties.len(), config.len());
  for (key, value) in config {
    assert_eq!(&properties[key]["default"], value, "default of {key}");
  }
}

#[test]
fn should_use_global_config() {
  let global_config = GlobalConfiguration {
    line_width: Some(120),
    indent_width: Some(4),
    ..Default::default()
  };
  let result = resolve_config(Default::default(), &global_config);
  assert!(result.diagnostics.is_empty());
  assert_eq!(result.config.line_width, 120);
  assert_eq!(result.config.indent_width, 4);
}

#[test]
fn should_fail_on_parse_error() {
  let err = format_text(&PathBuf::from("./file.nix"), "{ a = ; }\n", &default_config()).unwrap_err();
  let message = err.to_string();
  assert!(message.starts_with("Formatting failed: "), "{message}");
}

fn default_config() -> Configuration {
  resolve_config(Default::default(), &GlobalConfiguration::default()).config
}
