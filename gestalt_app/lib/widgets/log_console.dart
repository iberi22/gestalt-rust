import 'dart:convert';
import 'package:flutter/material.dart';

class LogConsole extends StatelessWidget {
  final List<dynamic> events;
  final ScrollController scrollController;

  const LogConsole({
    super.key,
    required this.events,
    required this.scrollController,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF1E1E1E),
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.all(8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Padding(
            padding: EdgeInsets.only(bottom: 8.0),
            child: Text(
              'TERMINAL OUTPUT',
              style: TextStyle(
                color: Colors.white54,
                fontSize: 10,
                letterSpacing: 1.5,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
          Expanded(
            child: ListView.builder(
              controller: scrollController,
              itemCount: events.length,
              itemBuilder: (context, index) {
                final event = events[index];
                final type = event['event_type'] ?? 'unknown';
                final timestamp = event['timestamp']?.toString().split('T').last.split('.').first ?? '';
                final payload = event['payload'] ?? event['metadata'] ?? {};

                Color typeColor = Colors.grey;
                if (type.contains('error')) typeColor = Colors.redAccent;
                if (type.contains('llm')) typeColor = Colors.purpleAccent;
                if (type.contains('project')) typeColor = Colors.blueAccent;

                return Padding(
                  padding: const EdgeInsets.symmetric(vertical: 2.0),
                  child: RichText(
                    text: TextSpan(
                      style: const TextStyle(fontFamily: 'Consolas', fontSize: 12),
                      children: [
                        TextSpan(text: '[$timestamp] ', style: const TextStyle(color: Colors.white38)),
                        TextSpan(text: '$type ', style: TextStyle(color: typeColor, fontWeight: FontWeight.bold)),
                        TextSpan(text: json.encode(payload), style: const TextStyle(color: Colors.white70)),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
