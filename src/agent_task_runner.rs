use futures::future::join_all;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PrReviewArgs {
    #[schemars(
        description = "Directory where review files will be created",
        example = "tmp/agent_reviews"
    )]
    pub output_dir: String,
}

async fn simulate_agent_review(
    agent_id: &str,
    output_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let file_id = Uuid::new_v4().to_string();
    let filename = format!("review_{}_{}.txt", agent_id, &file_id[..8]);
    let file_path = output_dir.join(filename);

    let dummy_review = format!(
        "PR Review by Agent: {}\n\
        =====================================\n\
        \n\
        ## Code Quality Assessment\n\
        - Overall code structure looks good\n\
        - Found 3 potential improvements\n\
        - No critical security issues detected\n\
        \n\
        ## Performance Analysis\n\
        - Memory usage within acceptable limits\n\
        - Runtime complexity looks efficient\n\
        \n\
        ## Recommendations\n\
        1. Consider adding more unit tests\n\
        2. Documentation could be improved\n\
        3. Error handling looks robust\n\
        \n\
        Agent ID: {}\n\
        Review Score: 8/10",
        agent_id, agent_id
    );

    let file_path_clone = file_path.clone();
    tokio::task::spawn_blocking(move || fs::write(&file_path_clone, dummy_review)).await??;

    let sleep_duration = Duration::from_millis(1500);
    tokio::time::sleep(sleep_duration).await;

    Ok(file_path)
}

#[derive(Clone)]
pub struct AgentTaskRunner {
    tool_router: ToolRouter<AgentTaskRunner>,
}

#[tool_router]
impl AgentTaskRunner {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Run parallel PR review with multiple LLM agents. Creates review files from different agent perspectives."
    )]
    async fn pr_review(
        &self,
        Parameters(args): Parameters<PrReviewArgs>,
    ) -> Result<CallToolResult, McpError> {
        let output_dir = PathBuf::from(&args.output_dir);

        let output_dir_clone = output_dir.clone();
        let result = match tokio::task::spawn_blocking(move || fs::create_dir_all(&output_dir_clone)).await {
            Ok(res) => res,
            Err(join_err) => return Err(McpError::internal_error(
                format!("Task join failed: {}", join_err),
                None,
            )),
        };
        if let Err(e) = result {
            return Err(McpError::internal_error(
                format!("Failed to create output directory: {}", e),
                None,
            ));
        }

        let agents = vec!["claude", "gpt4"];

        let tasks = agents
            .into_iter()
            .map(|agent_id| {
                let output_dir = output_dir.clone();
                tokio::spawn(async move { simulate_agent_review(agent_id, &output_dir).await })
            })
            .collect::<Vec<_>>();

        let results = join_all(tasks).await;
        let mut file_paths = Vec::new();
        let mut errors = Vec::new();

        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(Ok(path)) => {
                    file_paths.push(path.to_string_lossy().to_string());
                }
                Ok(Err(e)) => {
                    errors.push(format!("Agent {}: {}", i, e));
                }
                Err(e) => {
                    errors.push(format!("Task {}: {}", i, e));
                }
            }
        }

        if !errors.is_empty() && file_paths.is_empty() {
            return Err(McpError::internal_error(
                format!("All agents failed: {}", errors.join(", ")),
                None,
            ));
        }

        let result_text = if errors.is_empty() {
            format!(
                "PR Review Complete!\n\nGenerated {} review files:\n{}",
                file_paths.len(),
                file_paths.join("\n")
            )
        } else {
            format!(
                "PR Review Completed with some issues:\n\nGenerated {} review files:\n{}\n\nErrors:\n{}",
                file_paths.len(),
                file_paths.join("\n"),
                errors.join("\n")
            )
        };

        Ok(CallToolResult::success(vec![Content::text(result_text)]))
    }
}

#[tool_handler]
impl ServerHandler for AgentTaskRunner {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Multi-LLM Agent Task Runner server. Tools: run_rspec (run tests for a file), pr_review (run parallel PR review with multiple agents)."
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_task_runner_tools() {
        let router = AgentTaskRunner::new().tool_router;

        let tools = router.list_all();
        assert_eq!(tools.len(), 2);

        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(tool_names.contains(&"run_rspec"));
        assert!(tool_names.contains(&"pr_review"));
    }
}
