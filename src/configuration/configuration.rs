// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;
use serde::Serialize;

// NOTE:
// dprint is a customizable formatter platform. We should expose options here
// when upstream `nixfmt_rs` provides new formatting settings.
//
// In particular, we want to support `strict` mode once it becomes available.
// As of nixfmt_rs 0.5.0, the upstream CLI accepts `--strict` as a no-op,
// and the library API does not support it yet.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub line_width: u32,
  pub indent_width: u8,
}
