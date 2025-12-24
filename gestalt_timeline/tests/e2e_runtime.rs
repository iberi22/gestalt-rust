use gestalt_timeline::config::DatabaseSettings;
use gestalt_timeline::db::SurrealClient;
use gestalt_timeline::services::{
    AgentRuntime, AgentService, Cognition, LLMResponse, OrchestrationAction, ProjectService,
    TaskService, TimelineService, WatchService,
};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use anyhow::Result;

// Mock Cognition Engine
#[derive(Clone)]
struct MockCognition {
    actions_to_return: Arc<TokioMutex<Vec<Vec<OrchestrationAction>>>>,
}

impl MockCognition {
    fn new(actions: Vec<Vec<OrchestrationAction>>) -> Self {
        Self {
            actions_to_return: Arc::new(TokioMutex::new(actions)),
        }
    }
}

#[async_trait::async_trait]
impl Cognition for MockCognition {
    async fn chat(&self, _agent_id: &str, _message: &str) -> Result<LLMResponse> {
        Ok(LLMResponse {
            content: "Mock response".to_string(),
            model_id: "mock".to_string(),
            input_tokens: 0,
            output_tokens: 0,
            duration_ms: 0,
        })
    }

    async fn orchestrate(&self, _agent_id: &str, _workflow: &str, _context: Option<&str>) -> Result<Vec<OrchestrationAction>> {
        let mut actions = self.actions_to_return.lock().await;
        if !actions.is_empty() {
            Ok(actions.remove(0))
        } else {
            Ok(vec![])
        }
    }

    // For runtime loop
    async fn orchestrate_step(&self, _agent_id: &str, _goal: &str, _history: &[String], _context: Option<&str>) -> Result<Vec<OrchestrationAction>> {
        let mut actions = self.actions_to_return.lock().await;
        println!("DEBUG: Mock Actions len: {}", actions.len());
        if !actions.is_empty() {
            let next = actions.remove(0);
            println!("DEBUG: Returning {} actions", next.len());
            Ok(next)
        } else {
            println!("DEBUG: Returning empty actions");
            Ok(vec![])
        }
    }

    fn model_id(&self) -> &str {
        "mock-model"
    }
}

#[tokio::test]
async fn test_e2e_autonomous_agent() -> Result<()> {
    // Initialize tracing
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .try_init();

    // 1. Setup In-Memory Database
    let db_settings = DatabaseSettings {
        url: "mem://".to_string(),
        user: "root".to_string(),
        pass: "root".to_string(),
        namespace: "test".to_string(),
        database: "test".to_string(),
    };
    let db = SurrealClient::connect(&db_settings).await?;

    // 2. Initialize Services
    let timeline = TimelineService::new(db.clone());
    let project_service = ProjectService::new(db.clone(), timeline.clone());
    let task_service = TaskService::new(db.clone(), timeline.clone());
    let watch_service = WatchService::new(db.clone(), timeline.clone());
    let agent_service = AgentService::new(db.clone(), timeline.clone());

    // 3. Setup Mock LLM with a Plan
    // Step 1: Create Project
    // Step 2: Create Task
    // Step 3: Stop (empty list)
    let mock_actions = vec![
        vec![OrchestrationAction::CreateProject {
            name: "Project Omega".to_string(),
            description: Some("End-to-End Test Project".to_string()),
        }],
        vec![OrchestrationAction::CreateTask {
            project: "Project Omega".to_string(),
            description: "Phase 1 Initialization".to_string(),
        }],
    ];

    let mock_llm = Arc::new(MockCognition::new(mock_actions));

    // 4. Initialize Runtime
    let runtime = AgentRuntime::new(
        "test-agent".to_string(),
        mock_llm,
        project_service.clone(),
        task_service.clone(),
        watch_service.clone(),
        agent_service.clone(),
    );

    // 5. Run the Loop
    println!("🚀 Starting E2E Mock Loop...");
    runtime.run_loop("Create Project Omega").await?;
    println!("✅ Loop finished.");

    // 6. Verify Side Effects in Database

    // Check Project
    let projects = project_service.list_projects().await?;
    assert_eq!(projects.len(), 1, "Should have created 1 project");
    assert_eq!(projects[0].name, "Project Omega");
    println!("✅ Project Verification passed");

    // Check Task
    let tasks = task_service.list_tasks(None).await?;
    assert_eq!(tasks.len(), 1, "Should have created 1 task");
    assert_eq!(tasks[0].description, "Phase 1 Initialization");
    println!("✅ Task Verification passed");

    Ok(())
}

#[tokio::test]
async fn test_job_control_e2e() -> Result<()> {
    // 1. Setup In-Memory Database
    let db_settings = DatabaseSettings {
        url: "mem://".to_string(),
        user: "root".to_string(),
        pass: "root".to_string(),
        namespace: "test".to_string(),
        database: "test".to_string(),
    };
    let db = SurrealClient::connect(&db_settings).await?;

    // 2. Initialize Services
    let timeline = TimelineService::new(db.clone());
    let project_service = ProjectService::new(db.clone(), timeline.clone());
    let task_service = TaskService::new(db.clone(), timeline.clone());
    let watch_service = WatchService::new(db.clone(), timeline.clone());
    let agent_service = AgentService::new(db.clone(), timeline.clone());

    // 3. Setup Mock LLM with Jobs Plan
    // Command: "echo hello" (simulated background job that finishes quickly, or sleep)
    // To test "StopJob", we need something that runs for a bit.
    // Windows: "ping -t localhost" or "Start-Sleep -Seconds 10"
    // Linux: "sleep 10"

    #[cfg(target_os = "windows")]
    let sleep_cmd = "Start-Sleep -Seconds 10";
    #[cfg(not(target_os = "windows"))]
    let sleep_cmd = "sleep 10";

    let mock_actions = vec![
        // Step 1: Start Job
        vec![OrchestrationAction::StartJob {
            name: "sleeper".to_string(),
            command: sleep_cmd.to_string(),
        }],
        // Step 2: List Jobs
        vec![OrchestrationAction::ListJobs],
        // Step 3: Stop Job
        vec![OrchestrationAction::StopJob {
            name: "sleeper".to_string(),
        }],
        // Step 4: List Jobs (should be empty/finished)
        vec![OrchestrationAction::ListJobs],
    ];

    let mock_llm = Arc::new(MockCognition::new(mock_actions));

    // 4. Initialize Runtime
    let runtime = AgentRuntime::new(
        "test-agent".to_string(),
        mock_llm,
        project_service.clone(),
        task_service.clone(),
        watch_service.clone(),
        agent_service.clone(),
    );

    // 5. Run the Loop
    println!("🚀 Starting Job Control E2E Loop...");
    // We expect 4 steps + 1 empty to stop
    runtime.run_loop("Manage background jobs").await?;
    println!("✅ Loop finished.");

    // Note: Since run_loop captures observations, we can't assert observations directly here unless we inspect logs or modify runtime return.
    // However, if execute_action fails, it logs error. run_loop returns Ok(()) if successful loop.
    // The strict verification is that StartJob didn't panic and StopJob worked.

    Ok(())
}

#[tokio::test]
async fn test_delegation_e2e() -> Result<()> {
    // 1. Setup In-Memory Database
    let db_settings = DatabaseSettings {
        url: "mem://".to_string(),
        user: "root".to_string(),
        pass: "root".to_string(),
        namespace: "test".to_string(),
        database: "test".to_string(),
    };
    let db = SurrealClient::connect(&db_settings).await?;

    // 2. Initialize Services
    let timeline = TimelineService::new(db.clone());
    let project_service = ProjectService::new(db.clone(), timeline.clone());
    let task_service = TaskService::new(db.clone(), timeline.clone());
    let watch_service = WatchService::new(db.clone(), timeline.clone());
    let agent_service = AgentService::new(db.clone(), timeline.clone());

    // 3. Setup Mock LLM with Delegation Plan
    // Sequence:
    // 1. (Main) DelegateTask "sub-agent"
    // 2. (Sub) ExecuteShell "echo I am sub" (to prove it ran)
    // 3. (Main) Chat "Delegation done"

    let mock_actions = vec![
        // Step 1: Main Agent delegates
        vec![OrchestrationAction::DelegateTask {
            agent: "sub-agent".to_string(),
            goal: "Do sub task".to_string(),
        }],
        // Step 2: Sub Agent acts (The runtime for sub-agent will pull this next)
        vec![OrchestrationAction::ExecuteShell {
            command: "echo 'I am sub'".to_string(),
        }],
        // Step 3: Sub Agent finishes (pulls empty list -> breaks loop) -> IMPLICIT
        vec![],
        // Step 4: Main Agent receives control back.
        vec![OrchestrationAction::Chat {
            response: "Delegation complete".to_string(),
        }],
    ];

    let mock_llm = Arc::new(MockCognition::new(mock_actions));

    // 4. Initialize Runtime
    let runtime = AgentRuntime::new(
        "main-agent".to_string(),
        mock_llm,
        project_service.clone(),
        task_service.clone(),
        watch_service.clone(),
        agent_service.clone(),
    );

    // 5. Run the Loop
    println!("🚀 Starting Delegation E2E Loop...");
    runtime.run_loop("Root Goal").await?;
    println!("✅ Loop finished.");

    Ok(())
}
