import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import '../../application/sharing_cubit.dart';
import '../../domain/entities/shared_health_package.dart';

/// Page to receive health data from another OrionHealth node
class ReceivePage extends StatelessWidget {
  const ReceivePage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (_) => SharingCubit()..initialize(),
      child: const _ReceivePageContent(),
    );
  }
}

class _ReceivePageContent extends StatefulWidget {
  const _ReceivePageContent();

  @override
  State<_ReceivePageContent> createState() => _ReceivePageContentState();
}

class _ReceivePageContentState extends State<_ReceivePageContent> {
  @override
  void initState() {
    super.initState();
    // Start listening for incoming data
    WidgetsBinding.instance.addPostFrameCallback((_) {
      context.read<SharingCubit>().startListening(TransferMethod.nfc);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Recibir Datos'),
        actions: [
          IconButton(
            icon: const Icon(Icons.close),
            onPressed: () => Navigator.of(context).pop(),
          ),
        ],
      ),
      body: BlocConsumer<SharingCubit, SharingState>(
        listener: (context, state) {
          if (state is SharingReceiving && state.package != null) {
            _showPreviewDialog(context, state.package!);
          } else if (state is SharingComplete) {
            _showSuccessDialog(context, state.result);
          } else if (state is SharingError) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(state.message), backgroundColor: Colors.red),
            );
          }
        },
        builder: (context, state) {
          return _buildWaitingUI(state);
        },
      ),
    );
  }

  Widget _buildWaitingUI(SharingState state) {
    String message = 'Esperando datos...';
    IconData icon = Icons.wifi;
    Color color = Colors.blue;

    if (state is SharingScanning) {
      switch (state.method) {
        case TransferMethod.nfc:
          message = 'Acerca los dispositivos para recibir...';
          icon = Icons.nfc;
          break;
        case TransferMethod.ble:
          message = 'Buscando dispositivos Bluetooth...';
          icon = Icons.bluetooth;
          break;
        case TransferMethod.wifi:
          message = 'Esperando conexión WiFi...';
          icon = Icons.wifi;
          break;
      }
    } else if (state is SharingConnected) {
      message = 'Conexión establecida';
      color = Colors.orange;
    } else if (state is SharingTransferring) {
      message = state.message;
      color = Colors.blue;
    }

    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Container(
            padding: const EdgeInsets.all(32),
            decoration: BoxDecoration(
              color: color.withOpacity(0.1),
              shape: BoxShape.circle,
            ),
            child: Icon(icon, size: 80, color: color),
          ),
          const SizedBox(height: 32),
          Text(
            message,
            style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 16),
          const Text(
            'Los datos se recibirán de forma cifrada\ny se almacenarán en tu billetera de salud.',
            textAlign: TextAlign.center,
            style: TextStyle(color: Colors.grey),
          ),
          const SizedBox(height: 48),
          const CircularProgressIndicator(),
          const SizedBox(height: 16),
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancelar'),
          ),
        ],
      ),
    );
  }

  void _showPreviewDialog(BuildContext context, SharedHealthPackage package) {
    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (dialogContext) => AlertDialog(
        title: const Text('Datos recibidos'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('De: ${package.senderNodeId}'),
            Text('Tamaño: ${package.payload.encryptedData.length} bytes'),
            Text('Expira: ${package.timeRemaining.inMinutes}m ${package.timeRemaining.inSeconds % 60}s'),
            const Divider(),
            const Text('Categorías:', style: TextStyle(fontWeight: FontWeight.bold)),
            ...package.metadata.includedCategories.map(
              (c) => Text('• ${c.displayName}'),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.of(dialogContext).pop();
              context.read<SharingCubit>().rejectIncomingPackage();
            },
            child: const Text('Rechazar'),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.of(dialogContext).pop();
              context.read<SharingCubit>().acceptIncomingPackage();
            },
            child: const Text('Importar'),
          ),
        ],
      ),
    );
  }

  void _showSuccessDialog(BuildContext context, SharingResult result) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        icon: const Icon(Icons.check_circle, color: Colors.green, size: 64),
        title: const Text('¡Importación completa!'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('${result.bytesTransferred} bytes importados'),
            Text('Tiempo: ${result.transferTime.inSeconds}s'),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              Navigator.of(context).pop();
            },
            child: const Text('Listo'),
          ),
        ],
      ),
    );
  }
}
