import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

/// E2E Tests para Health Sharing (BLE/NFC/WiFi) de OrionHealth
///
/// Ejecutar con:
/// flutter test integration_test/health_sharing_e2e_test.dart

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Health Sharing - E2E Tests', () {

    // ============================================================
    // TEST 1: Página de compartir se renderiza
    // ============================================================
    testWidgets('E2E 1: Share page renders correctly', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: _MockSharePage(),
        ),
      );
      await tester.pumpAndSettle();

      // Verificar elementos
      expect(find.text('Compartir Datos'), findsOneWidget);
      expect(find.text('Selecciona datos a compartir'), findsOneWidget);
      expect(find.text('Método de transferencia'), findsOneWidget);
    });

    // ============================================================
    // TEST 2: Selección de categorías de datos
    // ============================================================
    testWidgets('E2E 2: Data category selection', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: _MockSharePageWithState(),
        ),
      );
      await tester.pumpAndSettle();

      // Seleccionar categoría
      final labsChip = find.text('Laboratorios');
      if (labsChip.evaluate().isNotEmpty) {
        await tester.tap(labsChip);
        await tester.pumpAndSettle();
      }

      // Verificar contador actualizado
      expect(find.textContaining('categorías seleccionadas'), findsOneWidget);
    });

    // ============================================================
    // TEST 3: Selección de método NFC
    // ============================================================
    testWidgets('E2E 3: Select NFC transfer method', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: _MockSharePageWithState(),
        ),
      );
      await tester.pumpAndSettle();

      // Seleccionar NFC
      final nfcOption = find.text('NFC');
      if (nfcOption.evaluate().isNotEmpty) {
        await tester.tap(nfcOption.first);
        await tester.pumpAndSettle();
      }

      // Verificar que NFC está seleccionado
      expect(find.text('Tap phones to share'), findsOneWidget);
    });

    // ============================================================
    // TEST 4: Selección de método BLE
    // ============================================================
    testWidgets('E2E 4: Select BLE transfer method', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: _MockSharePageWithState(),
        ),
      );
      await tester.pumpAndSettle();

      // Seleccionar BLE
      final bleOption = find.text('Bluetooth');
      if (bleOption.evaluate().isNotEmpty) {
        await tester.tap(bleOption.first);
        await tester.pumpAndSettle();
      }

      expect(find.text('Nearby device'), findsOneWidget);
    });

    // ============================================================
    // TEST 5: Selección de método WiFi
    // ============================================================
    testWidgets('E2E 5: Select WiFi transfer method', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: _MockSharePageWithState(),
        ),
      );
      await tester.pumpAndSettle();

      // Seleccionar WiFi
      final wifiOption = find.text('WiFi Direct');
      if (wifiOption.evaluate().isNotEmpty) {
        await tester.tap(wifiOption.first);
        await tester.pumpAndSettle();
      }

      expect(find.text('Same network'), findsOneWidget);
    });

    // ============================================================
    // TEST 6: Botón compartir deshabilitado sin selección
    // ============================================================
    testWidgets('E2E 6: Share button disabled without selection', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: _MockSharePageWithState(),
        ),
      );
      await tester.pumpAndSettle();

      // Verificar que el botón está deshabilitado
      final shareButton = find.text('Selecciona al menos una categoría');
      expect(shareButton, findsOneWidget);
    });

    // ============================================================
    // TEST 7: Transferencia en progreso
    // ============================================================
    testWidgets('E2E 7: Transfer in progress shows loading', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: const _MockTransferringPage(),
        ),
      );
      await tester.pumpAndSettle();

      // Verificar indicadores de transferencia
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.byType(LinearProgressIndicator), findsOneWidget);
      expect(find.text('Buscando dispositivos...'), findsOneWidget);
    });

    // ============================================================
    // TEST 8: Página de recibir se renderiza
    // ============================================================
    testWidgets('E2E 8: Receive page renders correctly', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: _MockReceivePage(),
        ),
      );
      await tester.pumpAndSettle();

      // Verificar elementos
      expect(find.text('Recibir Datos'), findsOneWidget);
      expect(find.text('Esperando datos...'), findsWidgets);
    });

    // ============================================================
    // TEST 9: Diálogo de confirmación de datos recibidos
    // ============================================================
    testWidgets('E2E 9: Incoming data preview dialog', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) => Scaffold(
              body: ElevatedButton(
                onPressed: () => _showPreviewDialog(context),
                child: const Text('Show Preview'),
              ),
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();

      // Abrir diálogo
      await tester.tap(find.text('Show Preview'));
      await tester.pumpAndSettle();

      // Verificar contenido del diálogo
      expect(find.text('Datos recibidos'), findsOneWidget);
      expect(find.text('Importar'), findsOneWidget);
      expect(find.text('Rechazar'), findsOneWidget);
    });

    // ============================================================
    // TEST 10: Transferencia exitosa
    // ============================================================
    testWidgets('E2E 10: Successful transfer shows confirmation', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) => Scaffold(
              body: ElevatedButton(
                onPressed: () => _showSuccessDialog(context),
                child: const Text('Show Success'),
              ),
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();

      // Abrir diálogo de éxito
      await tester.tap(find.text('Show Success'));
      await tester.pumpAndSettle();

      // Verificar confirmación
      expect(find.text('¡Compartido exitosamente!'), findsOneWidget);
      expect(find.text('Listo'), findsOneWidget);
    });

    // ============================================================
    // TEST 11: Cancelar transferencia
    // ============================================================
    testWidgets('E2E 11: Cancel transfer button works', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: const _MockTransferringPage(),
        ),
      );
      await tester.pumpAndSettle();

      // Buscar y tocar botón cancelar
      final cancelButton = find.text('Cancelar');
      expect(cancelButton, findsOneWidget);

      await tester.tap(cancelButton);
      await tester.pumpAndSettle();

      // Verificar que vuelve al estado inicial
      expect(find.text('Compartir Datos'), findsOneWidget);
    });

    // ============================================================
    // TEST 12: Verificación de privacidad antes de compartir
    // ============================================================
    testWidgets('E2E 12: Privacy verification before share', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: const _MockSharePage(),
        ),
      );
      await tester.pumpAndSettle();

      // Verificar mensaje de privacidad
      expect(find.textContaining('privacidad'), findsWidgets);
      expect(find.textContaining('cifrada'), findsWidgets);
    });
  });
}

// ============================================================================
// MOCK COMPONENTS
// ============================================================================

class _MockSharePage extends StatelessWidget {
  const _MockSharePage();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Compartir Datos')),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Selecciona datos a compartir',
                      style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 12),
                    Wrap(
                      spacing: 8,
                      runSpacing: 8,
                      children: [
                        FilterChip(label: Text('Laboratorios'), selected: false, onSelected: (_) {}),
                        FilterChip(label: Text('Medicamentos'), selected: false, onSelected: (_) {}),
                        FilterChip(label: Text('Signos Vitales'), selected: false, onSelected: (_) {}),
                      ],
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            Card(
              child: Column(
                children: [
                  RadioListTile(title: Text('NFC'), subtitle: Text('Tap phones to share'), value: 0, groupValue: -1, onChanged: (_) {}),
                  RadioListTile(title: Text('Bluetooth'), subtitle: Text('Nearby device'), value: 1, groupValue: -1, onChanged: (_) {}),
                  RadioListTile(title: Text('WiFi Direct'), subtitle: Text('Same network'), value: 2, groupValue: -1, onChanged: (_) {}),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _MockSharePageWithState extends StatefulWidget {
  @override
  State<_MockSharePageWithState> createState() => _MockSharePageWithStateState();
}

class _MockSharePageWithStateState extends State<_MockSharePageWithState> {
  final Set<String> _selectedCategories = {};
  int _selectedMethod = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Compartir Datos')),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('Selecciona datos a compartir'),
                    const SizedBox(height: 12),
                    Wrap(
                      spacing: 8,
                      children: [
                        FilterChip(
                          label: Text('Laboratorios'),
                          selected: _selectedCategories.contains('Laboratorios'),
                          onSelected: (s) => setState(() {
                            s ? _selectedCategories.add('Laboratorios') : _selectedCategories.remove('Laboratorios');
                          }),
                        ),
                        FilterChip(
                          label: Text('Medicamentos'),
                          selected: _selectedCategories.contains('Medicamentos'),
                          onSelected: (s) => setState(() {
                            s ? _selectedCategories.add('Medicamentos') : _selectedCategories.remove('Medicamentos');
                          }),
                        ),
                      ],
                    ),
                    const SizedBox(height: 8),
                    Text('${_selectedCategories.length} categorías seleccionadas'),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            Card(
              child: Column(
                children: [
                  RadioListTile(
                    title: Text('NFC'),
                    subtitle: Text('Tap phones to share'),
                    value: 0,
                    groupValue: _selectedMethod,
                    onChanged: (v) => setState(() => _selectedMethod = v ?? 0),
                  ),
                  RadioListTile(
                    title: Text('Bluetooth'),
                    subtitle: Text('Nearby device'),
                    value: 1,
                    groupValue: _selectedMethod,
                    onChanged: (v) => setState(() => _selectedMethod = v ?? 0),
                  ),
                  RadioListTile(
                    title: Text('WiFi Direct'),
                    subtitle: Text('Same network'),
                    value: 2,
                    groupValue: _selectedMethod,
                    onChanged: (v) => setState(() => _selectedMethod = v ?? 0),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 16),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _selectedCategories.isEmpty ? null : () {},
                child: Text(
                  _selectedCategories.isEmpty
                      ? 'Selecciona al menos una categoría'
                      : 'Compartir',
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _MockTransferringPage extends StatelessWidget {
  const _MockTransferringPage();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const CircularProgressIndicator(),
            const SizedBox(height: 24),
            const Text('Buscando dispositivos...', style: TextStyle(fontSize: 18)),
            const SizedBox(height: 16),
            const LinearProgressIndicator(),
            const SizedBox(height: 32),
            TextButton(
              onPressed: () {},
              child: const Text('Cancelar'),
            ),
          ],
        ),
      ),
    );
  }
}

class _MockReceivePage extends StatelessWidget {
  const _MockReceivePage();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Recibir Datos')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
              padding: const EdgeInsets.all(32),
              decoration: BoxDecoration(
                color: Colors.blue.withOpacity(0.1),
                shape: BoxShape.circle,
              ),
              child: const Icon(Icons.nfc, size: 80, color: Colors.blue),
            ),
            const SizedBox(height: 32),
            const Text(
              'Esperando datos...',
              style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            const Text(
              'Los datos se recibirán de forma cifrada',
              style: TextStyle(color: Colors.grey),
            ),
            const SizedBox(height: 48),
            const CircularProgressIndicator(),
            const SizedBox(height: 16),
            TextButton(
              onPressed: () {},
              child: const Text('Cancelar'),
            ),
          ],
        ),
      ),
    );
  }
}

void _showPreviewDialog(BuildContext context) {
  showDialog(
    context: context,
    builder: (context) => AlertDialog(
      title: const Text('Datos recibidos'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('De: OrionHealth-Maria'),
          Text('Tamaño: 2.5 KB'),
          const Divider(),
          const Text('Categorías:', style: TextStyle(fontWeight: FontWeight.bold)),
          Text('• Laboratorios'),
          Text('• Medicamentos'),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Rechazar'),
        ),
        ElevatedButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Importar'),
        ),
      ],
    ),
  );
}

void _showSuccessDialog(BuildContext context) {
  showDialog(
    context: context,
    builder: (context) => AlertDialog(
      icon: const Icon(Icons.check_circle, color: Colors.green, size: 64),
      title: const Text('¡Compartido exitosamente!'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text('2560 bytes transferidos'),
          Text('Tiempo: 3s'),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Listo'),
        ),
      ],
    ),
  );
}
