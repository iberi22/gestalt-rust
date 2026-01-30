// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// import 'package:flutter/material.dart';
// import 'package:gestalt_terminal/src/rust/api.dart'; // Standard path for V2
// import 'package:gestalt_terminal/src/rust/frb_generated.dart'; // Standard path for V2
//
// Future<void> main() async {
//   await RustLib.init();
//   runApp(const MyApp());
// }
//
// class MyApp extends StatelessWidget {
//   const MyApp({super.key});
//
//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       debugShowCheckedModeBanner: false,
//       theme: ThemeData.dark().copyWith(
//         scaffoldBackgroundColor: const Color(0xFF0A0A0A), // Deep black
//         textTheme: const TextTheme(
//           bodyMedium: TextStyle(
//             fontFamily: 'Consolas', // Or generic monospace
//             color: Color(0xFF00FF00), // Matrix green / Cyberpunk
//             fontSize: 14,
//           ),
//         ),
//       ),
//       home: const TerminalScreen(),
//     );
//   }
// }
//
// class TerminalScreen extends StatefulWidget {
//   const TerminalScreen({super.key});
//
//   @override
//   State<TerminalScreen> createState() => _TerminalScreenState();
// }
//
// class _TerminalScreenState extends State<TerminalScreen> {
//   String _greeting = "Initializing Bridge...";
//
//   @override
//   void initState() {
//     super.initState();
//     _fetchGreeting();
//   }
//
//   Future<void> _fetchGreeting() async {
//     try {
//       final message = await helloWorld();
//       setState(() {
//         _greeting = message;
//       });
//     } catch (e) {
//       setState(() {
//         _greeting = "Error: $e";
//       });
//     }
//   }
//
//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       body: Center(
//         child: Container(
//           padding: const EdgeInsets.all(20),
//           decoration: BoxDecoration(
//             border: Border.all(color: const Color(0xFF333333)),
//             color: const Color(0xFF0F0F0F),
//           ),
//           child: Column(
//             mainAxisSize: MainAxisSize.min,
//             children: [
//               const Text(
//                 ">_ GESTALT TERMINAL PROTOCOL",
//                 style: TextStyle(
//                   fontWeight: FontWeight.bold,
//                   letterSpacing: 2.0,
//                   color: Color(0xFF888888),
//                 ),
//               ),
//               const SizedBox(height: 20),
//               Text(
//                 _greeting,
//                 style: const TextStyle(
//                   fontFamily: 'Courier Inconsolata', // Fallback needed probably, relying on system mono
//                   fontSize: 24,
//                   shadows: [
//                      Shadow(
//                       blurRadius: 10.0,
//                       color: Color(0xFF00FF00),
//                       offset: Offset(0, 0),
//                     ),
//                   ],
//                 ),
//               ),
//             ],
//           ),
//         ),
//       ),
//     );
//   }
// }
//

import 'package:flutter/material.dart';
import 'package:gestalt_terminal/src/rust/api/simple.dart';
import 'package:gestalt_terminal/src/rust/frb_generated.dart';

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
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
            'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`',
          ),
        ),
      ),
    );
  }
}
