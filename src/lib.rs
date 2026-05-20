use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, LanguageServerId, Project,
    Result, Worktree,
};
use zed_extension_api::settings::{ContextServerSettings, LspSettings};

const CONTEXT_SERVER_ID: &str = "socket-security";
const LANGUAGE_SERVER_ID: &str = "socket-security-lsp";
const SOCKET_API_TOKEN_SETTING: &str = "socket_api_token";
const SOCKET_LEGACY_API_TOKEN_SETTING: &str = "api_token";
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
        worktree: &Worktree,
    ) -> Result<Command> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Err(format!(
                "Language server id must be `{LANGUAGE_SERVER_ID}`; saw `{language_server_id}`. Fix the language_servers entry in extension.toml.",
            ));
        }

        let lsp_settings = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)?;
        let mut env = Default::default();
        if let Some(token) = setting_token(lsp_settings.settings.as_ref()) {
            env = vec![("SOCKET_API_TOKEN".to_string(), token)];
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![extension_path(SOCKET_LSP_PATH)?],
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Err(format!(
                "Language server id must be `{LANGUAGE_SERVER_ID}`; saw `{language_server_id}`. Fix the language_servers entry in extension.toml.",
            ));
        }

        Ok(LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)?.settings)
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        self.language_server_initialization_options(language_server_id, worktree)
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
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

        let context_settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let mut env = context_settings
            .command
            .and_then(|command| command.env)
            .map(|env| env.into_iter().collect::<Vec<_>>())
            .unwrap_or_default();
        if let Some(token) = setting_token(context_settings.settings.as_ref()) {
            env.retain(|(key, _)| key != "SOCKET_API_TOKEN");
            env.push(("SOCKET_API_TOKEN".to_string(), token));
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![extension_path(MCP_REMOTE_PATH)?, SOCKET_MCP_URL.to_string()],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        if context_server_id.as_ref() != CONTEXT_SERVER_ID {
            return Err(format!(
                "Context server id must be `{CONTEXT_SERVER_ID}`; saw `{context_server_id}`. Fix the context_servers entry in extension.toml.",
            ));
        }

        Ok(Some(ContextServerConfiguration {
            installation_instructions: "Enter a Socket API token to enable authenticated MCP and package analysis requests. You can create a token from your Socket account settings. The token is stored in Zed settings under `context_servers.socket-security.settings.socket_api_token`.".to_string(),
            settings_schema: r#"{
  "type": "object",
  "properties": {
    "socket_api_token": {
      "type": "string",
      "title": "Socket API Token",
      "description": "Socket API token used by the MCP server and package diagnostics."
    },
    "api_token": {
      "type": "string",
      "title": "Socket API Token (legacy)",
      "description": "Legacy alias for socket_api_token."
    }
  },
  "additionalProperties": false
}"#.to_string(),
            default_settings: r#"{
  "socket_api_token": ""
}"#.to_string(),
        }))
    }
}

fn extension_path(relative_path: &str) -> Result<String> {
    let work_dir = env::current_dir().map_err(|err| {
        format!(
            "Current extension work directory must be readable; saw `{err}`. Reinstall the Socket Security extension in Zed.",
        )
    })?;
    let work_path = work_dir.join(relative_path);
    if work_path.exists() {
        return Ok(work_path.to_string_lossy().to_string());
    }
    let installed_path = work_dir
        .parent()
        .and_then(|extensions_dir| extensions_dir.parent())
        .map(|extensions_dir| {
            extensions_dir
                .join("installed")
                .join(CONTEXT_SERVER_ID)
                .join(relative_path)
        })
        .filter(|path| path.exists());
    if let Some(installed_path) = installed_path {
        return Ok(installed_path.to_string_lossy().to_string());
    }

    Ok(work_path.to_string_lossy().to_string())
}

fn setting_token(settings: Option<&zed::serde_json::Value>) -> Option<String> {
    let settings = settings?;
    for key in [SOCKET_API_TOKEN_SETTING, SOCKET_LEGACY_API_TOKEN_SETTING] {
        if let Some(token) = settings.get(key).and_then(|value| value.as_str()) {
            let token = token.trim();
            if !token.is_empty() {
                return Some(token.to_string());
            }
        }
    }
    None
}

zed::register_extension!(SocketSecurityExtension);
