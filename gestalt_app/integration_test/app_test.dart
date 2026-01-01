import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:gestalt_app/main.dart' as app;
import 'package:gestalt_app/screens/main_layout.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('verify settings screen navigation and content', (WidgetTester tester) async {
    // Start the app
    app.main();
    await tester.pumpAndSettle();

    // Verify initial "System Overview" is present (Dashboard)
    expect(find.text('System Overview'), findsOneWidget);

    // Find the Settings icon/text in the sidebar and tap it
    // The sidebar has a ListTile with "Settings" text
    final settingsButton = find.text('Settings');
    expect(settingsButton, findsOneWidget);

    await tester.tap(settingsButton);
    await tester.pumpAndSettle();

    // Verify "System Settings" header is present
    expect(find.text('System Settings'), findsOneWidget);

    // Verify "AI Model Configuration" section exists
    expect(find.text('AI Model Configuration'), findsOneWidget);

    // Verify Dropdown is present
    // We look for the hint text "Select a model" or the dropdown widget itself
    expect(find.byType(DropdownButton<String>), findsOneWidget);

    // Note: Interacting with the dropdown requires more complex setup if the backend isn't running
    // For this E2E test, confirming the UI elements loaded is sufficient proof of integration
  });
}
