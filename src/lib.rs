use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerId, LanguageServerId, Project, Result, Worktree,
};

const CONTEXT_SERVER_ID: &str = "socket-security";
const LANGUAGE_SERVER_ID: &str = "socket-security-lsp";
const MCP_REMOTE_PACKAGE: &str = "mcp-remote";
const MCP_REMOTE_VERSION: &str = "0.1.38";
const MCP_REMOTE_PATH: &str = "node_modules/mcp-remote/dist/proxy.js";
const SOCKET_MCP_URL: &str = "https://mcp.socket.dev/";
const SOCKET_LSP_PATH: &str = "src/zed-lsp/server";

struct SocketSecurityExtension;

impl zed::Extension for SocketSecurityExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Err(format!(
                "Language server id must be `{LANGUAGE_SERVER_ID}`; saw `{language_server_id}`. Fix the language_servers entry in extension.toml.",
            ));
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![extension_path(SOCKET_LSP_PATH)?],
            env: Default::default(),
        })
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        if context_server_id.as_ref() != CONTEXT_SERVER_ID {
            return Err(format!(
                "Context server id must be `{CONTEXT_SERVER_ID}`; saw `{context_server_id}`. Fix the context_servers entry in extension.toml.",
            ));
        }

        let installed_version = zed::npm_package_installed_version(MCP_REMOTE_PACKAGE)?;
        if installed_version.as_deref() != Some(MCP_REMOTE_VERSION) {
            zed::npm_install_package(MCP_REMOTE_PACKAGE, MCP_REMOTE_VERSION)?;
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![extension_path(MCP_REMOTE_PATH)?, SOCKET_MCP_URL.to_string()],
            env: Default::default(),
        })
    }
}

fn extension_path(relative_path: &str) -> Result<String> {
    Ok(env::current_dir()
        .map_err(|err| {
            format!(
                "Current extension work directory must be readable; saw `{err}`. Reinstall the Socket Security extension in Zed.",
            )
        })?
        .join(relative_path)
        .to_string_lossy()
        .to_string())
}

zed::register_extension!(SocketSecurityExtension);
