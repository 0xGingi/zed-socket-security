use std::env;
use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

const CONTEXT_SERVER_ID: &str = "socket-security";
const MCP_REMOTE_PACKAGE: &str = "mcp-remote";
const MCP_REMOTE_VERSION: &str = "0.1.38";
const MCP_REMOTE_PATH: &str = "node_modules/mcp-remote/dist/proxy.js";
const SOCKET_MCP_URL: &str = "https://mcp.socket.dev/";

struct SocketSecurityExtension;

impl zed::Extension for SocketSecurityExtension {
    fn new() -> Self {
        Self
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

        let proxy_path = env::current_dir()
            .map_err(|err| {
                format!(
                    "Current extension work directory must be readable; saw `{err}`. Reinstall the Socket Security extension in Zed.",
                )
            })?
            .join(MCP_REMOTE_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![proxy_path, SOCKET_MCP_URL.to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SocketSecurityExtension);
