import { createRequire } from 'node:module'

import { describe, expect, test } from 'vitest'

const require = createRequire(import.meta.url)
const { findRefs } = require('../src/zed-lsp/scanner')

interface PackageRef {
  purl: string
}

export function refs(filename: string, text: string, languageId = 'toml') {
  return findRefs({
    languageId,
    text,
    uri: `file:///workspace/${filename}`,
  }) as PackageRef[]
}

describe('zed lsp scanner', () => {
  test('finds Cargo.toml dependency tables', () => {
    expect(
      refs(
        'Cargo.toml',
        `
[package]
name = "app"

[dependencies]
serde = "1"
tokio = { version = "1", features = ["rt"] }
"tower-http" = "0.6"

[dev-dependencies]
proptest = "1"

[target.'cfg(unix)'.dependencies]
nix = "0.29"

[workspace.dependencies]
anyhow = "1"
`,
      ).map(ref => ref.purl),
    ).toEqual([
      'pkg:cargo/serde',
      'pkg:cargo/tokio',
      'pkg:cargo/tower-http',
      'pkg:cargo/proptest',
      'pkg:cargo/nix',
      'pkg:cargo/anyhow',
    ])
  })

  test('finds Cargo.lock package entries', () => {
    expect(
      refs(
        'Cargo.lock',
        `
[[package]]
name = "serde"
version = "1.0.0"

[[package]]
name = "tokio"
version = "1.0.0"
`,
      ).map(ref => ref.purl),
    ).toEqual(['pkg:cargo/serde', 'pkg:cargo/tokio'])
  })

  test('finds Rust source crate imports', () => {
    expect(
      refs(
        'lib.rs',
        `
use serde::Deserialize;
pub use tokio::runtime::Runtime;
extern crate proc_macro;
extern crate anyhow;
`,
        'rust',
      ).map(ref => ref.purl),
    ).toEqual(['pkg:cargo/serde', 'pkg:cargo/tokio', 'pkg:cargo/anyhow'])
  })
})
