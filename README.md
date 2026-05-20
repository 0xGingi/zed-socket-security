# Socket Security Zed Extension

This extension exposes the hosted Socket Security MCP server to Zed's Agent Panel.

## Features

- Registers a Socket Security context server in Zed.
- Bridges Zed's stdio MCP extension interface to `https://mcp.socket.dev/` with `mcp-remote`.

## Development

Install Rust with `rustup`, then install the repository as a dev extension from Zed with `zed: install dev extension`.

```bash
RUSTC="$(rustup which rustc --toolchain stable)" rustup run stable cargo build --target wasm32-wasip1 --release
```

## Migration Notes

The previous VS Code extension also rendered inline package decorations, hover cards, a login command, a status bar item, and editor configuration. Zed's current extension API supports languages, debuggers, themes, icon themes, snippets, and MCP servers, so those VS Code-specific features do not have a Zed extension surface yet.

The TypeScript implementation remains in `src/` as migration reference code. The active Zed extension entry point is `src/lib.rs`.
