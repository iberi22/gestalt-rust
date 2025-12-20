import 'package:flutter/material.dart';
import 'package:gestalt_app/src/rust/api/simple.dart';
import 'package:gestalt_app/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('Gestalt Agent')),
        body: const AgentScreen(),
      ),
    );
  }
}

class AgentScreen extends StatefulWidget {
  const AgentScreen({super.key});

  @override
  State<AgentScreen> createState() => _AgentScreenState();
}

class _AgentScreenState extends State<AgentScreen> {
  final TextEditingController _controller = TextEditingController();
  String _response = "Waiting for input...";
  bool _loading = false;

  Future<void> _sendPrompt() async {
    setState(() {
      _loading = true;
      _response = "Thinking...";
    });

    try {
      // Call Rust
      final response = await askAgent(prompt: _controller.text);

      setState(() {
        _response = "Response: ${response.textResponse}\n\nUI Component: ${response.uiComponent?.widgetType}";
      });

      // Print full object to console for debugging
      print("RUST RESPONSE: $response");

    } catch (e) {
      setState(() {
        _response = "Error: $e";
      });
    } finally {
      setState(() {
        _loading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          TextField(
            controller: _controller,
            decoration: const InputDecoration(labelText: 'Ask Gestalt'),
          ),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: _loading ? null : _sendPrompt,
            child: const Text('Send'),
          ),
          const SizedBox(height: 24),
          Text(_response),
        ],
      ),
    );
  }
}
