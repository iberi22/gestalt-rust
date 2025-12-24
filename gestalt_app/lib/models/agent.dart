class Agent {
  final String id;
  final String name;
  final String status;
  final String type;
  final DateTime lastSeen;

  Agent({
    required this.id,
    required this.name,
    required this.status,
    required this.type,
    required this.lastSeen,
  });

  factory Agent.fromJson(Map<String, dynamic> json) {
    return Agent(
      id: json['id']?.toString() ?? 'unknown',
      name: json['name'] ?? 'Unknown Agent',
      status: json['status'] ?? 'offline',
      type: json['agent_type'] ?? 'cli',
      lastSeen: DateTime.tryParse(json['last_seen'].toString()) ?? DateTime.now(),
    );
  }
}
