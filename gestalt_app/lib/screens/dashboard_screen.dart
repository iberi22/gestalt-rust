import 'dart:async';
import 'package:flutter/material.dart';
import '../services/api_service.dart';
import '../models/agent.dart';
import '../models/project.dart';
import '../widgets/project_card.dart';
import '../widgets/agent_status_pill.dart';
import '../widgets/log_console.dart';

class DashboardScreen extends StatefulWidget {
  const DashboardScreen({super.key});

  @override
  State<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  final ApiService _api = ApiService();
  final ScrollController _logController = ScrollController();
  final TextEditingController _inputController = TextEditingController();

  List<Project> _projects = [];
  List<Agent> _agents = [];
  List<dynamic> _logs = [];
  Timer? _pollingTimer;
  bool _isLoading = false;

  @override
  void initState() {
    super.initState();
    _refreshData();
    _pollingTimer = Timer.periodic(const Duration(seconds: 2), (timer) {
      _refreshData();
    });
  }

  @override
  void dispose() {
    _pollingTimer?.cancel();
    _logController.dispose();
    _inputController.dispose();
    super.dispose();
  }

  Future<void> _refreshData() async {
    final projects = await _api.getProjects();
    final agents = await _api.getAgents();
    final logs = await _api.getTimeline();

    if (mounted) {
      setState(() {
        _projects = projects;
        _agents = agents;
        _logs = logs;
      });

      // Auto-scroll logs if at bottom
      if (_logController.hasClients && _logController.offset >= _logController.position.maxScrollExtent - 50) {
        _logController.animateTo(
           _logController.position.maxScrollExtent,
           duration: const Duration(milliseconds: 300),
           curve: Curves.easeOut
        );
      }
    }
  }

  Future<void> _sendCommand() async {
    if (_inputController.text.isEmpty) return;
    setState(() => _isLoading = true);
    await _api.sendGoal(_inputController.text);
    _inputController.clear();
    await Future.delayed(const Duration(milliseconds: 500));
    await _refreshData();
    setState(() => _isLoading = false);
  }

  @override
  Widget build(BuildContext context) {
    // DashboardScreen is now just the content area. Navigation is handled by MainLayout.
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text("System Overview", style: TextStyle(color: Colors.white, fontSize: 28, fontWeight: FontWeight.bold)),
              Row(
                children: _agents.map((a) => Padding(
                  padding: const EdgeInsets.only(left: 8.0),
                  child: AgentStatusPill(agent: a),
                )).toList(),
              ),
            ],
          ),
          const SizedBox(height: 24),

          // Projects Grid (Horizontal Preview)
          const Text("Recent Projects", style: TextStyle(color: Colors.white70, fontSize: 16)),
          const SizedBox(height: 12),
          SizedBox(
            height: 160,
            child: _projects.isEmpty
              ? _emptyState("No active projects")
              : ListView.separated(
                  scrollDirection: Axis.horizontal,
                  itemCount: _projects.length,
                  separatorBuilder: (_, __) => const SizedBox(width: 16),
                  itemBuilder: (context, index) => SizedBox(
                    width: 280,
                    child: ProjectCard(project: _projects[index]),
                  ),
                ),
          ),

          const SizedBox(height: 24),

          // Command Input
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            decoration: BoxDecoration(
              color: const Color(0xFF2C2C2C),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: Colors.white10),
            ),
            child: Row(
              children: [
                const Icon(Icons.terminal, color: Colors.blueAccent),
                const SizedBox(width: 12),
                Expanded(
                  child: TextField(
                    controller: _inputController,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(
                      hintText: "Enter a command or goal for the agent...",
                      hintStyle: TextStyle(color: Colors.white30),
                      border: InputBorder.none,
                    ),
                    onSubmitted: (_) => _sendCommand(),
                  ),
                ),
                IconButton(
                  icon: _isLoading
                    ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                    : const Icon(Icons.send, color: Colors.blueAccent),
                  onPressed: _isLoading ? null : _sendCommand,
                ),
              ],
            ),
          ),

          const SizedBox(height: 24),

          // Logs
          const Text("System Logs", style: TextStyle(color: Colors.white70, fontSize: 16)),
          const SizedBox(height: 12),
          Expanded(
            child: LogConsole(events: _logs, scrollController: _logController),
          ),
        ],
      ),
    );
  }



  Widget _emptyState(String message) {
    return Container(
      width: double.infinity,
      decoration: BoxDecoration(
        color: const Color(0xFF1E1E1E),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white10),
      ),
      child: Center(
        child: Text(message, style: const TextStyle(color: Colors.white30)),
      ),
    );
  }
}
