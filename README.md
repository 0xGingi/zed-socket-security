# Socket Security Zed Extension

This extension exposes the hosted Socket Security MCP server to Zed's Agent Panel.

## Features

- Registers a Socket Security context server in Zed.
- Bridges Zed's stdio MCP extension interface to `https://mcp.socket.dev/` with `mcp-remote`.
- Starts a Socket Security language server for JavaScript, TypeScript, Python, Go, JSON, TOML, and YAML files.
- Reports package alerts as LSP diagnostics and renders Socket package data in hover markdown.

## Development

Install Rust with `rustup`, then install the repository as a dev extension from Zed with `zed: install dev extension`.

```bash
RUSTC="$(rustup which rustc --toolchain stable)" rustup run stable cargo build --target wasm32-wasip1 --release
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

Restart Zed after installing so it reloads the extension index. This path installs `extension.wasm` directly and does not rely on Zed compiling the Rust extension.

## Configuration

The language server reads Socket credentials from `SOCKET_API_TOKEN`. The legacy aliases `SOCKET_API_KEY`, `SOCKET_SECURITY_API_TOKEN`, and `SOCKET_SECURITY_API_KEY` are also accepted for one-cycle compatibility.

## Migration Notes

The previous VS Code extension also rendered inline package decorations, hover cards, a login command, a status bar item, and editor configuration. Zed's current extension API supports languages, debuggers, themes, icon themes, snippets, and MCP servers, so those VS Code-specific features do not have a Zed extension surface yet.

Package diagnostics and hovers are now provided through LSP, which is how Zed extensions can surface editor feedback. A Zed-native status bar login control and VS Code authentication-provider equivalent are still not available through the current extension API.
