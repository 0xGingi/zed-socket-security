# Socket Security Zed Extension

This extension exposes the hosted Socket Security MCP server to Zed's Agent Panel.

## Features

- Registers a Socket Security context server in Zed.
- Bridges Zed's stdio MCP extension interface to `https://mcp.socket.dev/` with `mcp-remote`.
- Starts a Socket Security language server for JavaScript, TypeScript, TSX, Python, Go, Rust, JSON, JSONC, TOML, and YAML files.
- Reports package alerts as LSP diagnostics and renders Socket package data in hover markdown.
- Scans `package.json`, `requirements.txt`, `*-requirements.txt`, `pyproject.toml`, `go.mod`, `Cargo.toml`, `Cargo.lock`, and source imports for npm, PyPI, Go, and Cargo package references.
- Exposes a Zed Configure flow for `socket_api_token`.

## Development

Install Rust with `rustup`, then build the Zed extension Wasm component with the same target Zed uses internally:

```bash
RUSTC="$(rustup which rustc --toolchain stable)" rustup run stable cargo build --target wasm32-wasip2 --release
```

## Packaging

Build a non-dev Zed extension archive and manifest:

```bash
node zed-package
```

Install the precompiled extension into Zed's normal extension directory:

```bash
node zed-package --install
```

Restart Zed after installing so it reloads the extension index. This path installs a precompiled `extension.wasm` component and does not rely on Zed compiling the Rust extension.

The installer writes:

- `build/zed/dist/archive.tar.gz`
- `build/zed/dist/manifest.json`
- `build/zed/package/extension.wasm`
- `~/Library/Application Support/Zed/extensions/installed/socket-security`
- `~/Library/Application Support/Zed/extensions/work/socket-security/src/zed-lsp`

Zed shows local non-registry extensions in the dev/local section of the Extensions view. The Rebuild button is not needed after this install path.

## Configuration

Open the extension in Zed's Extensions view and choose Configure. Enter your Socket API token in `socket_api_token`.

The setting is stored under `context_servers.socket-security.settings.socket_api_token` in Zed settings. Package diagnostics also read that value, so the same token powers both the MCP server and LSP package analysis.

The language server still accepts `SOCKET_API_TOKEN` from the process environment. The legacy aliases `api_token`, `SOCKET_API_KEY`, `SOCKET_SECURITY_API_TOKEN`, and `SOCKET_SECURITY_API_KEY` are also accepted for one-cycle compatibility.

## Testing

After `node zed-package --install`, restart Zed and open a project containing one of:

- `package.json`
- `requirements.txt`
- `pyproject.toml`
- `go.mod`
- `Cargo.toml`
- `Cargo.lock`
- JavaScript, TypeScript, TSX, Python, Go, or Rust source imports

Hover a package name or import to see Socket package data. Packages with Socket alerts should also produce LSP diagnostics.

To debug loading issues, run `zed: open log` and search for `socket-security`, `socket-security-lsp`, or `extension_host`.
