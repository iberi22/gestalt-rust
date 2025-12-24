import 'package:flutter/material.dart';
import 'screens/main_layout.dart'; // Changed from dashboard_screen.dart

void main() {
  runApp(const GestaltApp());
}

class GestaltApp extends StatelessWidget {
  const GestaltApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Gestalt Agent Dashboard',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blueAccent,
          brightness: Brightness.dark,
          background: const Color(0xFF121212),
        ),
        useMaterial3: true,
        fontFamily: 'Segoe UI',
      ),
      home: const MainLayout(), // Changed from DashboardScreen
    );
  }
}
