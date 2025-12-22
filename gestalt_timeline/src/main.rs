//! Gestalt Timeline CLI Entry Point

use clap::Parser;
use gestalt_timeline::cli::{Cli, Commands};
use gestalt_timeline::db::SurrealClient;
use gestalt_timeline::services::{
    AgentService, LLMService, OrchestrationAction, ProjectService,
    TaskService, TimelineService, WatchService
};
use surrealdb::sql::Thing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Helper to convert Option<Thing> to String for display
fn thing_to_string(thing: &Option<Thing>) -> String {
    thing.as_ref().map(|t| t.to_string()).unwrap_or_else(|| "unknown".to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize database connection
    let db = SurrealClient::connect().await?;

    // Initialize services
    let timeline_service = TimelineService::new(db.clone());
    let project_service = ProjectService::new(db.clone(), timeline_service.clone());
    let task_service = TaskService::new(db.clone(), timeline_service.clone());
    let watch_service = WatchService::new(db.clone(), timeline_service.clone());
    let agent_service = AgentService::new(db.clone(), timeline_service.clone());

    // Get agent ID from environment or use default
    let agent_id = std::env::var("GESTALT_AGENT_ID").unwrap_or_else(|_| "cli_default".to_string());

    // Execute command
    match cli.command {
        Commands::AddProject { name } => {
            let project = project_service.create_project(&name, &agent_id).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&project)?);
            } else {
                println!("✅ Project created: {} (ID: {})", project.name, thing_to_string(&project.id));
            }
        }

        Commands::AddTask { project, description } => {
            let task = task_service.create_task(&project, &description, &agent_id).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&task)?);
            } else {
                println!("✅ Task created: {} (ID: {})", task.description, task.id);
            }
        }

        Commands::RunTask { task_id } => {
            let result = task_service.run_task(&task_id, &agent_id).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("🚀 Task {} completed: {:?}", task_id, result.status);
            }
        }

        Commands::ListProjects => {
            let projects = project_service.list_projects().await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&projects)?);
            } else {
                if projects.is_empty() {
                    println!("📋 No projects found.");
                } else {
                    println!("📋 Projects:");
                    for p in projects {
                        println!("  • {} [{}] - {}", p.name, p.status, thing_to_string(&p.id));
                    }
                }
            }
        }

        Commands::ListTasks { project } => {
            let tasks = task_service.list_tasks(project.as_deref()).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&tasks)?);
            } else {
                if tasks.is_empty() {
                    println!("📋 No tasks found.");
                } else {
                    println!("📋 Tasks:");
                    for t in tasks {
                        println!("  • [{}] {} - {}", t.status, t.description, t.id);
                    }
                }
            }
        }

        Commands::Status { project } => {
            let status = project_service.get_status(&project).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&status)?);
            } else {
                println!("📊 Project: {}", status.name);
                println!("   Status: {}", status.status);
                println!("   Tasks: {} total, {} completed", status.total_tasks, status.completed_tasks);
                println!("   Progress: {}%", status.progress_percent);
            }
        }

        Commands::Timeline { since } => {
            let events = timeline_service.get_timeline(since.as_deref()).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&events)?);
            } else {
                if events.is_empty() {
                    println!("🕐 No events in timeline.");
                } else {
                    println!("🕐 Timeline:");
                    for e in events {
                        println!("  {} | {} | {} | {}",
                            e.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                            e.agent_id,
                            e.event_type,
                            e.id
                        );
                    }
                }
            }
        }

        Commands::Watch { project, events } => {
            // Parse event filter
            let event_filter = events.map(|e| {
                e.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>()
            });

            // Setup Ctrl+C handler
            let watch_service_clone = watch_service.clone();
            tokio::spawn(async move {
                tokio::signal::ctrl_c().await.ok();
                watch_service_clone.stop();
            });

            // Start watching (this blocks until stopped)
            watch_service
                .start_watching(&agent_id, project.as_deref(), event_filter)
                .await?;
        }

        Commands::Broadcast { message, project } => {
            let event = watch_service
                .broadcast_message(&agent_id, &message, project.as_deref())
                .await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&event)?);
            } else {
                println!("📢 Broadcast sent: {}", message);
            }
        }

        Commands::Subscribe { project } => {
            // Get project ID first
            let proj = project_service.get_by_name(&project).await?;
            if let Some(p) = proj {
                println!("📡 Subscribed to project: {} ({})", project, thing_to_string(&p.id));

                // Setup Ctrl+C handler
                let watch_service_clone = watch_service.clone();
                tokio::spawn(async move {
                    tokio::signal::ctrl_c().await.ok();
                    watch_service_clone.stop();
                });

                // Start watching with project filter
                let project_id_str = thing_to_string(&p.id);
                watch_service
                    .start_watching(&agent_id, Some(&project_id_str), None)
                    .await?;
            } else {
                println!("❌ Project not found: {}", project);
            }
        }

        Commands::AgentConnect { name } => {
            let agent = agent_service.connect(&agent_id, name.as_deref()).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&agent)?);
            } else {
                println!("🤖 Agent connected: {} ({})", agent.name, agent.agent_type);
            }
        }

        Commands::AgentDisconnect => {
            agent_service.disconnect(&agent_id).await?;
            if cli.json {
                println!(r#"{{"agent_id": "{}", "status": "disconnected"}}"#, agent_id);
            } else {
                println!("👋 Agent disconnected: {}", agent_id);
            }
        }

        Commands::ListAgents { online } => {
            let agents = if online {
                agent_service.list_online_agents().await?
            } else {
                agent_service.list_agents().await?
            };
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&agents)?);
            } else {
                if agents.is_empty() {
                    println!("🤖 No agents found.");
                } else {
                    println!("🤖 Agents:");
                    for a in agents {
                        println!("  • {} [{}] ({}) - last seen: {}",
                            a.name,
                            a.status,
                            a.agent_type,
                            a.last_seen.format("%H:%M:%S")
                        );
                    }
                }
            }
        }

        Commands::AiChat { message } => {
            // Initialize LLM service
            let llm_service = LLMService::new(db.clone(), timeline_service.clone()).await?;

            println!("🤖 Sending message to Claude Sonnet 4.5...");
            let response = llm_service.chat(&agent_id, &message).await?;

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&response)?);
            } else {
                println!("\n💬 Claude:\n{}", response.content);
                println!("\n📊 Tokens: {} in / {} out", response.input_tokens, response.output_tokens);
            }
        }

        Commands::AiOrchestrate { workflow, project, dry_run } => {
            // Initialize LLM service
            let llm_service = LLMService::new(db.clone(), timeline_service.clone()).await?;

            println!("🎯 Orchestrating workflow with Claude Sonnet 4.5...");
            let actions = llm_service.orchestrate(&agent_id, &workflow, project.as_deref()).await?;

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&actions)?);
            } else {
                println!("\n📋 Planned Actions:");
                for (i, action) in actions.iter().enumerate() {
                    match action {
                        OrchestrationAction::CreateProject { name, description } => {
                            println!("  {}. Create project: '{}' {}", i + 1, name,
                                description.as_ref().map(|d| format!("({})", d)).unwrap_or_default());
                        }
                        OrchestrationAction::CreateTask { project, description } => {
                            println!("  {}. Create task in '{}': {}", i + 1, project, description);
                        }
                        OrchestrationAction::RunTask { task_id } => {
                            println!("  {}. Run task: {}", i + 1, task_id);
                        }
                        OrchestrationAction::ListProjects => {
                            println!("  {}. List all projects", i + 1);
                        }
                        OrchestrationAction::ListTasks { project } => {
                            println!("  {}. List tasks{}", i + 1,
                                project.as_ref().map(|p| format!(" for '{}'", p)).unwrap_or_default());
                        }
                        OrchestrationAction::GetStatus { project } => {
                            println!("  {}. Get status of '{}'", i + 1, project);
                        }
                        OrchestrationAction::Chat { response } => {
                            println!("  {}. 💬 {}", i + 1, response);
                        }
                    }
                }

                if dry_run {
                    println!("\n⚠️  Dry run mode - no actions executed");
                } else {
                    println!("\n🚀 Executing actions...");
                    for action in actions {
                        match action {
                            OrchestrationAction::CreateProject { name, .. } => {
                                match project_service.create_project(&name, &agent_id).await {
                                    Ok(p) => println!("  ✅ Created project: {}", p.name),
                                    Err(e) => println!("  ❌ Failed to create project: {}", e),
                                }
                            }
                            OrchestrationAction::CreateTask { project, description } => {
                                match task_service.create_task(&project, &description, &agent_id).await {
                                    Ok(t) => println!("  ✅ Created task: {}", t.description),
                                    Err(e) => println!("  ❌ Failed to create task: {}", e),
                                }
                            }
                            OrchestrationAction::RunTask { task_id } => {
                                match task_service.run_task(&task_id, &agent_id).await {
                                    Ok(r) => println!("  ✅ Task completed: {:?}", r.status),
                                    Err(e) => println!("  ❌ Failed to run task: {}", e),
                                }
                            }
                            OrchestrationAction::ListProjects => {
                                let projects = project_service.list_projects().await?;
                                println!("  📋 {} projects found", projects.len());
                            }
                            OrchestrationAction::ListTasks { project } => {
                                let tasks = task_service.list_tasks(project.as_deref()).await?;
                                println!("  📋 {} tasks found", tasks.len());
                            }
                            OrchestrationAction::GetStatus { project } => {
                                match project_service.get_status(&project).await {
                                    Ok(s) => println!("  📊 {}: {}% complete", s.name, s.progress_percent),
                                    Err(e) => println!("  ❌ Failed to get status: {}", e),
                                }
                            }
                            OrchestrationAction::Chat { response } => {
                                println!("  💬 {}", response);
                            }
                        }
                    }
                    println!("\n✅ Orchestration complete!");
                }
            }
        }
    }

    // Explicitly drop services and database client for graceful shutdown
    // This helps minimize SurrealDB background task cancellation warnings
    drop(project_service);
    drop(task_service);
    drop(watch_service);
    drop(agent_service);
    drop(timeline_service);
    drop(db);

    Ok(())
}

