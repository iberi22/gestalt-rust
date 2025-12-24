import 'package:flutter/material.dart';
import 'dashboard_screen.dart';
import 'project_list_screen.dart'; // We'll create this next
import '../widgets/agent_status_pill.dart'; // If needed in header

class MainLayout extends StatefulWidget {
  const MainLayout({super.key});

  @override
  State<MainLayout> createState() => _MainLayoutState();
}

class _MainLayoutState extends State<MainLayout> {
  int _selectedIndex = 0;

  final List<Widget> _screens = [
    const DashboardScreen(),
    const ProjectListScreen(), // Placeholder for now
    const Center(child: Text("Agents View (Coming Soon)", style: TextStyle(color: Colors.white))),
    const Center(child: Text("Settings View (Coming Soon)", style: TextStyle(color: Colors.white))),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF121212),
      body: Row(
        children: [
          // Sidebar
          Container(
            width: 250,
            color: const Color(0xFF1E1E1E),
            child: Column(
              children: [
                const SizedBox(height: 30),
                const Icon(Icons.hub, size: 50, color: Colors.blueAccent),
                const SizedBox(height: 10),
                const Text("GESTALT", style: TextStyle(color: Colors.white, fontSize: 20, letterSpacing: 2, fontWeight: FontWeight.bold)),
                const SizedBox(height: 40),
                _navItem(Icons.dashboard, "Dashboard", 0),
                _navItem(Icons.folder, "Projects", 1),
                _navItem(Icons.memory, "Agents", 2),
                _navItem(Icons.settings, "Settings", 3),
              ],
            ),
          ),
          // Main Content Area
          Expanded(
            child: _screens[_selectedIndex],
          ),
        ],
      ),
    );
  }

  Widget _navItem(IconData icon, String label, int index) {
    final isSelected = _selectedIndex == index;
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
      decoration: BoxDecoration(
        color: isSelected ? Colors.blueAccent.withOpacity(0.1) : Colors.transparent,
        borderRadius: BorderRadius.circular(8),
      ),
      child: ListTile(
        leading: Icon(icon, color: isSelected ? Colors.blueAccent : Colors.white54),
        title: Text(label, style: TextStyle(color: isSelected ? Colors.blueAccent : Colors.white54)),
        selected: isSelected,
        onTap: () => setState(() => _selectedIndex = index),
      ),
    );
  }
}
