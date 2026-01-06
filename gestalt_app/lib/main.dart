import 'package:flutter/material.dart';
import 'screens/main_layout.dart';
import 'theme/neo_glass_theme.dart';

void main() {
  runApp(const GestaltApp());
}

class GestaltApp extends StatelessWidget {
  const GestaltApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Neural-Link v2.0',
      debugShowCheckedModeBanner: false,
      theme: NeoGlassTheme.lightTheme,
      darkTheme: NeoGlassTheme.darkTheme,
      themeMode: ThemeMode.dark,
      home: const MainLayout(),
    );
  }
}
