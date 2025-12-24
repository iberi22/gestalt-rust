use anyhow::Result;
use tracing::info;
use crate::services::{
    AgentService, LLMService, ProjectService, TaskService, WatchService, OrchestrationAction, Cognition
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// The Autonomous Agent Runtime.
/// Encapsulates the "Think-Act-Observe" loop.
#[derive(Clone)]
pub struct AgentRuntime {
    agent_id: String,
    llm: Arc<dyn Cognition>,
    project: ProjectService,
    task: TaskService,
    watch: WatchService,
    agent: AgentService,
    max_steps: usize,
    // Background jobs: Name -> Child Process
    jobs: Arc<Mutex<HashMap<String, tokio::process::Child>>>,
}

impl AgentRuntime {
    pub fn new(
        agent_id: String,
        llm: Arc<dyn Cognition>,
        project: ProjectService,
        task: TaskService,
        watch: WatchService,
        agent: AgentService,
    ) -> Self {
        Self {
            agent_id,
            llm,
            project,
            task,
            watch,
            agent,
            max_steps: 10, // Default safety limit
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Run the autonomous loop for a specific goal.
    pub async fn run_loop(&self, goal: &str) -> Result<()> {
        info!("🔄 Starting Autonomous Loop for Agent: {}", self.agent_id);
        info!("🎯 Goal: {}", goal);

        // Update Agent Status to Busy
        use crate::services::AgentStatus;
        let _ = self.agent.set_status(&self.agent_id, AgentStatus::Busy).await;

        let mut conversation_history = Vec::new(); // TODO: Use proper Bedrock Message types

        // Initial Context for the LLM
        let initial_prompt = format!("GOAL: {}\n\nPlease start working on this goal.", goal);
        conversation_history.push(initial_prompt);

        for step in 0..self.max_steps {
            info!("📍 Step {}/{}", step + 1, self.max_steps);

            // 1. THINK: Helper to get next action based on history
            let actions = self.llm.orchestrate_step(&self.agent_id, goal, &conversation_history, None).await?;

            if actions.is_empty() {
                info!("✅ Agent decided to stop (no actions).");
                break;
            }

            // 2. ACT: Execute actions
            for action in actions {
                let action_summary = format!("Action: {:?}", action);
                conversation_history.push(action_summary.clone());

                let observation = match self.execute_action(&action).await {
                    Ok(obs) => obs,
                    Err(e) => {
                        println!("❌ Execution Error: {:?}", e);
                        format!("Error: {}", e)
                    }
                };

                // 3. OBSERVE: Create observation for history
                let observation_msg = format!("Observation: {}", observation);
                info!("👀 {}", observation_msg);
                conversation_history.push(observation_msg);
            }
        }

        // Update Agent Status to Idle
        let _ = self.agent.set_status(&self.agent_id, AgentStatus::Idle).await;

        Ok(())
    }

    async fn execute_action(&self, action: &OrchestrationAction) -> Result<String> {
        match action {
            OrchestrationAction::CreateProject { name, description: _description } => {
                self.project.create_project(name, &self.agent_id).await?;
                Ok(format!("Created Project '{}'", name))
            }
            OrchestrationAction::CreateTask { project, description } => {
                self.task.create_task(project, description, &self.agent_id).await?;
                Ok(format!("Created Task in '{}'", project))
            }
            OrchestrationAction::RunTask { task_id } => {
                self.task.run_task(task_id, &self.agent_id).await?;
                Ok(format!("Ran Task '{}'", task_id))
            }
            OrchestrationAction::ListProjects => {
                let projects = self.project.list_projects().await?;
                Ok(format!("Listed {} Projects", projects.len()))
            }
            OrchestrationAction::ListTasks { project } => {
                let tasks = self.task.list_tasks(project.as_deref()).await?;
                Ok(format!("Listed {} Tasks", tasks.len()))
            }
            OrchestrationAction::GetStatus { project } => {
                let status = self.project.get_status(project).await?;
                Ok(format!("Project '{}' is {}% complete", project, status.progress_percent))
            }
            OrchestrationAction::Chat { response } => {
                info!("💬 Agent says: {}", response);
                Ok(format!("Agent said: {}", response))
            }
            OrchestrationAction::ReadFile { path } => {
                info!("📂 Reading file: {}", path);
                match tokio::fs::read_to_string(path).await {
                    Ok(content) => Ok(format!("File '{}' content:\n{}", path, content)),
                    Err(e) => Ok(format!("Error reading file '{}': {}", path, e)),
                }
            }
            OrchestrationAction::WriteFile { path, content } => {
                info!("💾 Writing file: {}", path);
                if let Some(parent) = std::path::Path::new(path).parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                match tokio::fs::write(path, content).await {
                    Ok(_) => Ok(format!("Successfully wrote to file '{}'", path)),
                    Err(e) => Ok(format!("Error writing file '{}': {}", path, e)),
                }
            }
            OrchestrationAction::ExecuteShell { command } => {
                info!("🐚 Executing command: {}", command);
                // Security Note: This runs commands directly on the host shell.
                // In production, this should be sandboxed or restricted.

                #[cfg(target_os = "windows")]
                let mut cmd = tokio::process::Command::new("powershell");
                #[cfg(target_os = "windows")]
                cmd.arg("-Command").arg(command);

                #[cfg(not(target_os = "windows"))]
                let mut cmd = tokio::process::Command::new("sh");
                #[cfg(not(target_os = "windows"))]
                cmd.arg("-c").arg(command);

                match cmd.output().await {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let exit_code = output.status.code().unwrap_or(-1);
                        Ok(format!(
                            "Command executed (Exit: {})\nSTDOUT:\n{}\nSTDERR:\n{}",
                            exit_code, stdout, stderr
                        ))
                    }
                    Err(e) => Ok(format!("Failed to execute command '{}': {}", command, e)),
                }
            }
            OrchestrationAction::StartJob { name, command } => {
                info!("🏃 Starting background job '{}': {}", name, command);

                #[cfg(target_os = "windows")]
                let mut cmd = tokio::process::Command::new("powershell");
                #[cfg(target_os = "windows")]
                cmd.arg("-Command").arg(command);

                #[cfg(not(target_os = "windows"))]
                let mut cmd = tokio::process::Command::new("sh");
                #[cfg(not(target_os = "windows"))]
                cmd.arg("-c").arg(command);

                // Spawn and detach (sort of, we keep handle)
                // We must pipe output if we want to read it later, but for now let's inherit or null
                cmd.stdout(std::process::Stdio::piped());
                cmd.stderr(std::process::Stdio::piped());

                match cmd.spawn() {
                    Ok(child) => {
                        let id = child.id().unwrap_or(0);
                        let mut jobs = self.jobs.lock().await;
                        jobs.insert(name.clone(), child);
                        Ok(format!("Job '{}' started successfully (PID: {}).", name, id))
                    }
                    Err(e) => Ok(format!("Failed to start job '{}': {}", name, e)),
                }
            }
            OrchestrationAction::StopJob { name } => {
                info!("🛑 Stopping job '{}'", name);
                let mut jobs = self.jobs.lock().await;
                if let Some(mut child) = jobs.remove(name) {
                    match child.kill().await {
                        Ok(_) => Ok(format!("Job '{}' stopped.", name)),
                        Err(e) => Ok(format!("Failed to stop job '{}': {}", name, e)),
                    }
                } else {
                    Ok(format!("Job '{}' not found.", name))
                }
            }
            OrchestrationAction::ListJobs => {
                let jobs = self.jobs.lock().await;
                if jobs.is_empty() {
                    Ok("No background jobs running.".to_string())
                } else {
                    let mut output = String::from("Running Jobs:\n");
                    for (name, child) in jobs.iter() {
                        output.push_str(&format!("- {} (PID: {:?})\n", name, child.id()));
                    }
                    Ok(output)
                }
            }
            OrchestrationAction::DelegateTask { agent, goal } => {
                info!("🤝 Delegating task to '{}' agent: {}", agent, goal);

                // Create a sub-agent runtime (clone services)
                // distinct agent_id for the sub-agent
                let sub_agent_id = format!("{}-{}", self.agent_id, agent);

                let sub_runtime = AgentRuntime::new(
                    sub_agent_id.clone(),
                    self.llm.clone(),
                    self.project.clone(),
                    self.task.clone(),
                    self.watch.clone(),
                    self.agent.clone(),
                );

                // Connect the sub-agent formally
                let _ = self.agent.connect(&sub_agent_id, Some(agent)).await;

                // Execute recursively
                // We must box the future to avoid infinite size recursion error
                let sub_future = async move {
                    sub_runtime.run_loop(goal).await
                };

                match Box::pin(sub_future).await {
                    Ok(_) => Ok(format!("Delegated task to '{}' completed successfully.", agent)),
                    Err(e) => Ok(format!("Delegated task to '{}' failed: {}", agent, e)),
                }
            }
        }
    }
}
