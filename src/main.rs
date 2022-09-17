use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

const LANGUAGE: &str = "elixir";

fn initialize(params: InitializeParams) -> Result<()> {
    let mut server_args = vec![];

    // Check for user specified LSP server path
    // ```
    // [lapce-elixir-ls.lsp]
    // serverPath = "[path or filename]"
    // serverArgs = ["--arg1", "--arg2"]
    // ```
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(args) = lsp.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        PLUGIN_RPC.start_lsp(
                            Url::parse(&format!("urn:{}", server_path))?,
                            server_args,
                            vec![DocumentFilter {
                                language: Some(LANGUAGE.to_string()),
                                scheme: None,
                                pattern: None,
                            }],
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    // Architecture check
    let _ = match VoltEnvironment::architecture().as_deref() {
        Ok("x86_64") => "x86_64",
        Ok("aarch64") => "aarch64",
        _ => return Ok(()),
    };

    // OS check
    let _ = match VoltEnvironment::operating_system().as_deref() {
        Ok("macos") => "macos",
        Ok("linux") => "linux",
        Ok("windows") => "windows",
        _ => return Ok(()),
    };

    // Download URL
    // let _ = format!("https://github.com/elixir-lsp/elixir-ls/releases/download/v0.11.0/elixir-ls-1.11-22.3.zip");

    // see lapce_plugin::Http for available API to download files

    let filename = match VoltEnvironment::operating_system().as_deref() {
        Ok("windows") => {
            format!("{}.bat", "elixir-ls-release/language_server")
        }
        _ => "elixir-ls-release/language_server.sh".to_string(),
    };

    // Plugin working directory
    let volt_uri = VoltEnvironment::uri()?;
    let server_path = Url::parse(&volt_uri)?.join(&filename)?;

    // Available language IDs
    // https://github.com/lapce/lapce/blob/HEAD/lapce-proxy/src/buffer.rs#L173
    PLUGIN_RPC.start_lsp(
        server_path,
        server_args,
        vec![DocumentFilter {
                                language: Some(LANGUAGE.to_string()),
                                scheme: None,
                                pattern: None,
                            }],
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}
