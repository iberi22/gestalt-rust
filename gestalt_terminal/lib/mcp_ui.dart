import 'package:flutter/material.dart';
import 'package:gestalt_terminal/src/rust/api/mcp.dart';

class McpUiRenderer extends StatelessWidget {
  final List<McpComponent> components;

  const McpUiRenderer({super.key, required this.components});

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      padding: const EdgeInsets.all(16.0),
      itemCount: components.length,
      itemBuilder: (context, index) {
        return _buildComponent(context, components[index]);
      },
    );
  }

  Widget _buildComponent(BuildContext context, McpComponent component) {
    return component.map(
      card: (c) => Card(
        color: Colors.grey[900],
        margin: const EdgeInsets.only(bottom: 16.0),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                c.title,
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.greenAccent,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                c.content,
                style: const TextStyle(color: Colors.white70),
              ),
            ],
          ),
        ),
      ),
      button: (b) => Padding(
        padding: const EdgeInsets.symmetric(vertical: 4.0),
        child: ElevatedButton(
          onPressed: () {
            // TODO: Send action back to Rust
            debugPrint("Action: ${b.actionId}");
          },
          style: ElevatedButton.styleFrom(
            backgroundColor: Colors.blueAccent.withOpacity(0.2),
            foregroundColor: Colors.blueAccent,
          ),
          child: Text(b.label),
        ),
      ),
      markdown: (m) => Padding(
        padding: const EdgeInsets.only(bottom: 16.0),
        child: Text(
          m.content,
          style: const TextStyle(fontFamily: 'monospace', color: Colors.white),
        ),
      ),
      row: (r) => Row(
        children: r.children.map((c) => Expanded(child: _buildComponent(context, c))).toList(),
      ),
      column: (c) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: c.children.map((child) => _buildComponent(context, child)).toList(),
      ),
    );
  }
}
