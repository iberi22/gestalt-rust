use gestalt_timeline::db::SurrealClient;
use gestalt_timeline::services::MemoryService;
use gestalt_timeline::models::EventType;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_memory_service_surreal_native() {
    let db = SurrealClient::connect_mem().await.expect("Failed to connect to mem db");
    let memory_service = MemoryService::new(db.clone());

    let agent_id = "test_agent";
    let content = "The secret code is 12345";
    let context = "confidential";
    let tags = vec!["secret".to_string(), "code".to_string()];

    // Simple embedding (mock)
    let embedding = Some(vec![0.1; 384]);

    // Save memory
    let saved = memory_service.save(agent_id, content, context, tags.clone(), embedding.clone()).await.expect("Failed to save memory");
    assert_eq!(saved.agent_id, agent_id);
    assert_eq!(saved.content, content);

    // Search by content/tags (lexical fallback)
    let results = memory_service.search("secret", Some(agent_id), 10, None).await.expect("Failed to search memory");
    assert!(!results.is_empty());
    assert_eq!(results[0].content, content);

    // Search by vector similarity
    let results_vec = memory_service.search("query", Some(agent_id), 10, embedding).await.expect("Failed vector search");
    assert!(!results_vec.is_empty());
    assert_eq!(results_vec[0].content, content);
}

#[tokio::test]
async fn test_live_query_coordination() {
    let db = SurrealClient::connect_mem().await.expect("Failed to connect to mem db");

    use gestalt_timeline::models::TimelineEvent;
    use futures::StreamExt;

    let mut stream = db.subscribe::<TimelineEvent>("timeline_events").await.expect("Failed to subscribe");

    let agent_id = "agent_a";
    let event = TimelineEvent::new(agent_id, EventType::ChatMessage).with_payload(serde_json::json!({"text": "hello"}));

    let db_clone = db.clone();
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        let _ = db_clone.create("timeline_events", &event).await.expect("Failed to create event");
    });

    if let Some(notification) = stream.next().await {
        let notification = notification.expect("Notification error");
        assert_eq!(notification.data.agent_id, agent_id);
    } else {
        panic!("Stream closed without receiving event");
    }
}
