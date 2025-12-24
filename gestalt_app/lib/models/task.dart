class Task {
  final String id;
  final String description;
  final String projectId;
  final String status;

  Task({
    required this.id,
    required this.description,
    required this.projectId,
    required this.status,
  });

  factory Task.fromJson(Map<String, dynamic> json) {
     String idStr = 'unknown';
    if (json['id'] is Map) {
      idStr = json['id']['id']['String'] ?? json['id'].toString();
    } else {
      idStr = json['id'].toString();
    }

    // Project ID might be a Thing or string
    String projId = 'unknown';
    if (json['project_id'] is Map) {
      projId = json['project_id']['id']['String'] ?? json['project_id'].toString();
    } else {
      projId = json['project_id']?.toString() ?? 'unknown';
    }

    return Task(
      id: idStr,
      description: json['description'] ?? 'No description',
      projectId: projId,
      status: json['status'] ?? 'pending',
    );
  }
}
