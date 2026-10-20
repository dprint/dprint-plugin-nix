// SPDX-License-Identifier: MPL-2.0

use std::path::Path;

use crate::configuration::Configuration;

/// Error that occurred while formatting Nix text.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
  /// The text could not be parsed by nixfmt-rs.
  #[error("Formatting failed: {0}")]
  Nixfmt(String),
}

pub fn format_text(_file_path: &Path, input_text: &str, config: &Configuration) -> Result<Option<String>, FormatError> {
  let formatted = nixfmt_rs::format_with(input_text, &build_options(config))
    .map_err(|err| FormatError::Nixfmt(nixfmt_rs::format_error(input_text, None, &err)))?;
  Ok(if formatted == input_text { None } else { Some(formatted) })
}

fn build_options(config: &Configuration) -> nixfmt_rs::Options {
  // `Options` is non-exhaustive, so it can't be created with a struct expression
  let mut options = nixfmt_rs::Options::default();
  options.width = config.line_width as usize;
  options.indent = config.indent_width as usize;
  options
}
