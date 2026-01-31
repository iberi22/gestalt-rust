import 'package:flutter/material.dart';
import 'package:xterm/xterm.dart';
import 'package:gestalt_terminal/src/rust/api/terminal.dart';
import 'package:gestalt_terminal/src/rust/api/mcp.dart';
import 'package:gestalt_terminal/src/rust/frb_generated.dart';
import 'package:gestalt_terminal/mcp_ui.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Gestalt Terminal',
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF0A0A0A),
        scrollbarTheme: ScrollbarThemeData(
           thumbColor: MaterialStateProperty.all(Colors.grey[800]),
        ),
      ),
      home: const MainScreen(),
    );
  }
}

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  // xterm terminal
  final Terminal _terminal = Terminal(maxLines: 10000);
  late final TerminalController _terminalController;

  // MCP UI components
  final List<McpComponent> _mcpComponents = [];

  @override
  void initState() {
    super.initState();
    _terminalController = TerminalController();

    // Listen to input from the terminal (user typing) and send to PTY
    _terminal.onOutput = (data) {
      sendTerminalInput(input: data);
    };

    _initStreams();

    // Simulate initial agent event to show capabilities
    // Delayed slightly to allow UI to settle
    Future.delayed(const Duration(seconds: 1), () {
      simulateAgentEvent(eventType: "analysis");
    });
  }

  Future<void> _initStreams() async {
    // Initialize Terminal PTY
    // Note: initTerminal returns a stream of output from the shell
    final terminalStream = initTerminal();
    terminalStream.listen((data) {
      _terminal.write(data);
    });

    // Initialize MCP UI Stream
    final mcpStream = streamMcpUi();
    mcpStream.listen((component) {
      setState(() {
        _mcpComponents.add(component);
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          // Terminal Area (Left)
          Expanded(
            flex: 2,
            child: Container(
              color: Colors.black,
              padding: const EdgeInsets.all(8),
              child: TerminalView(
                _terminal,
                controller: _terminalController,
                autofocus: true,
                theme: TerminalThemes.defaultTheme,
              ),
            ),
          ),

          // Divider
          const VerticalDivider(width: 1, color: Colors.grey),

          // MCP UI Area (Right)
          Expanded(
            flex: 1,
            child: Column(
              children: [
                Container(
                    width: double.infinity,
                    padding: const EdgeInsets.all(12),
                    alignment: Alignment.centerLeft,
                    color: const Color(0xFF1A1A1A),
                    child: const Text("AGENT CONTEXT", style: TextStyle(fontWeight: FontWeight.bold, letterSpacing: 1.2))
                ),
                Expanded(
                  child: McpUiRenderer(components: _mcpComponents),
                ),
                // Test buttons for demonstration
                 Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: Wrap(
                    spacing: 8,
                    children: [
                      ElevatedButton(
                        onPressed: () => simulateAgentEvent(eventType: "analysis"),
                        child: const Text("Sim Analysis")
                      ),
                       ElevatedButton(
                        onPressed: () => simulateAgentEvent(eventType: "action"),
                        child: const Text("Sim Action")
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
