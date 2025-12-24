class Project {
  final String id;
  final String name;
  final String status;
  final DateTime createdAt;

  Project({
    required this.id,
    required this.name,
    required this.status,
    required this.createdAt,
  });

  factory Project.fromJson(Map<String, dynamic> json) {
    // Handle SurrealDB ID format which might be {tb: ..., id: ...} or just string
    String idStr = 'unknown';
    if (json['id'] is Map) {
      idStr = json['id']['id']['String'] ?? json['id'].toString();
    } else {
      idStr = json['id'].toString();
    }

    return Project(
      id: idStr,
      name: json['name'] ?? 'Unnamed',
      status: json['status'] ?? 'active',
      createdAt: DateTime.parse(json['created_at'] ?? DateTime.now().toIso8601String()),
    );
  }
}
