import 'package:flutter/material.dart';
import '../services/api_service.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({super.key});

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  final ApiService _api = ApiService();
  List<String> _models = [];
  String? _selectedModel;
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadModels();
  }

  Future<void> _loadModels() async {
    final models = await _api.getModels();
    setState(() {
      _models = models;
      // Default to known Sonnet if list empty or not set
      if (_selectedModel == null && _models.isNotEmpty) {
        _selectedModel = _models.firstWhere(
            (m) => m.contains("sonnet") && m.contains("3-5"),
            orElse: () => _models.first);
      }
      _isLoading = false;
    });
  }

  Future<void> _updateModel(String? newValue) async {
    if (newValue == null) return;
    setState(() {
      _selectedModel = newValue;
    });

    final success = await _api.setModel(newValue);
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(success ? "Model updated to $newValue" : "Failed to update model"),
          backgroundColor: success ? Colors.green : Colors.red,
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent, // Inherits from MainLayout
      body: Padding(
        padding: const EdgeInsets.all(32.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              "System Settings",
              style: TextStyle(color: Colors.white, fontSize: 32, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 32),
            Container(
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                color: const Color(0xFF1E1E1E),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: Colors.white10),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                   const Text(
                    "AI Model Configuration",
                    style: TextStyle(color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 8),
                  const Text(
                    "Select the active Bedrock Foundation Model for the coding agent.",
                    style: TextStyle(color: Colors.white54, fontSize: 14),
                  ),
                  const SizedBox(height: 24),
                  _isLoading
                      ? const CircularProgressIndicator()
                      : Container(
                          padding: const EdgeInsets.symmetric(horizontal: 16),
                          decoration: BoxDecoration(
                            color: const Color(0xFF2C2C2C),
                            borderRadius: BorderRadius.circular(8),
                            border: Border.all(color: Colors.white24),
                          ),
                          child: DropdownButtonHideUnderline(
                            child: DropdownButton<String>(
                              value: _selectedModel,
                              isExpanded: true,
                              dropdownColor: const Color(0xFF2C2C2C),
                              style: const TextStyle(color: Colors.white, fontSize: 16),
                              icon: const Icon(Icons.arrow_drop_down, color: Colors.blueAccent),
                              items: _models.map((String model) {
                                return DropdownMenuItem<String>(
                                  value: model,
                                  child: Text(model),
                                );
                              }).toList(),
                              onChanged: _updateModel,
                              hint: const Text("Select a model", style: TextStyle(color: Colors.white38)),
                            ),
                          ),
                        ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
