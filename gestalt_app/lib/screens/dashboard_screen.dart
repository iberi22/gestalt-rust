import 'dart:async';
import 'package:flutter/material.dart';
import 'package:glass_kit/glass_kit.dart';
import 'package:flutter_animate/flutter_animate.dart';
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
    _pollingTimer = Timer.periodic(const Duration(seconds: 4), (timer) {
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
    try {
      final projects = await _api.getProjects();
      final agents = await _api.getAgents();
      final logs = await _api.getTimeline();

      if (mounted) {
        setState(() {
          _projects = projects;
          _agents = agents;
          _logs = logs;
        });

        if (_logController.hasClients && _logController.offset >= _logController.position.maxScrollExtent - 50) {
          _logController.animateTo(
             _logController.position.maxScrollExtent,
             duration: const Duration(milliseconds: 300),
             curve: Curves.easeOut
          );
        }
      }
    } catch (e) {
      debugPrint("Error refreshing dashboard: $e");
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
    return Padding(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    "System Architecture",
                    style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                  ),
                  const Text(
                    "Monitoring local and cloud neural nodes",
                    style: TextStyle(color: Colors.white54, fontSize: 16),
                  ),
                ],
              ),
              Row(
                children: [
                  ..._agents.map((a) => Padding(
                    padding: const EdgeInsets.only(left: 12.0),
                    child: AgentStatusPill(agent: a),
                  )),
                ],
              ),
            ],
          ).animate().fadeIn(duration: 600.ms).slideY(begin: -0.1, end: 0),

          const SizedBox(height: 40),

          // Primary Actions Row
          Row(
            children: [
              Expanded(
                child: _buildInfoCard(
                  "ACTIVE AGENTS",
                  "${_agents.length}",
                  Icons.psychology,
                  Colors.purpleAccent,
                ),
              ),
              const SizedBox(width: 24),
              Expanded(
                child: _buildInfoCard(
                  "CONNECTED REPOS",
                  "${_projects.length}",
                  Icons.account_tree,
                  Colors.blueAccent,
                ),
              ),
              const SizedBox(width: 24),
              Expanded(
                child: _buildInfoCard(
                  "SYSTEM STATUS",
                  "OPTIMAL",
                  Icons.verified_user,
                  Colors.greenAccent,
                ),
              ),
            ],
          ).animate().fadeIn(delay: 200.ms).slideY(begin: 0.1, end: 0),

          const SizedBox(height: 40),

          // Command Bar
          GlassContainer.clearGlass(
            height: 70,
            width: double.infinity,
            borderRadius: BorderRadius.circular(16),
            borderWidth: 1,
            borderColor: Colors.white.withOpacity(0.1),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Row(
                children: [
                  const Icon(Icons.flash_on, color: Colors.orangeAccent),
                  const SizedBox(width: 16),
                  Expanded(
                    child: TextField(
                      controller: _inputController,
                      style: const TextStyle(color: Colors.white, fontSize: 18),
                      decoration: const InputDecoration(
                        hintText: "Issue a new directives to the swarm...",
                        hintStyle: TextStyle(color: Colors.white24),
                        border: InputBorder.none,
                      ),
                      onSubmitted: (_) => _sendCommand(),
                    ),
                  ),
                  if (_isLoading)
                    const CircularProgressIndicator(color: Colors.purpleAccent, strokeWidth: 2)
                  else
                    IconButton(
                      icon: const Icon(Icons.send_rounded, color: Colors.purpleAccent),
                      onPressed: _sendCommand,
                    ),
                ],
              ),
            ),
          ).animate().fadeIn(delay: 400.ms).scale(begin: const Offset(0.95, 0.95)),

          const SizedBox(height: 40),

          // Log Terminal
          Expanded(
            child: GlassContainer.clearGlass(
              width: double.infinity,
              borderRadius: BorderRadius.circular(24),
              borderWidth: 1,
              borderColor: Colors.white.withOpacity(0.05),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Row(
                      children: [
                        const Icon(Icons.terminal, color: Colors.greenAccent, size: 20),
                        const SizedBox(width: 12),
                        const Text(
                          "NEURAL LOG STREAM",
                          style: TextStyle(color: Colors.greenAccent, fontWeight: FontWeight.bold, letterSpacing: 1),
                        ),
                      ],
                    ),
                  ),
                  const Divider(color: Colors.white10, height: 1),
                  Expanded(
                    child: LogConsole(events: _logs, scrollController: _logController),
                  ),
                ],
              ),
            ),
          ).animate().fadeIn(delay: 600.ms),
        ],
      ),
    );
  }

  Widget _buildInfoCard(String title, String value, IconData icon, Color color) {
    return GlassContainer.clearGlass(
      height: 120,
      borderRadius: BorderRadius.circular(20),
      borderWidth: 1,
      borderColor: color.withOpacity(0.2),
      child: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Row(
          children: [
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: color.withOpacity(0.1),
                shape: BoxShape.circle,
              ),
              child: Icon(icon, color: color, size: 28),
            ),
            const SizedBox(width: 20),
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(title, style: const TextStyle(color: Colors.white54, fontSize: 12, fontWeight: FontWeight.bold)),
                Text(value, style: const TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
