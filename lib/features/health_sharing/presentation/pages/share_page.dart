import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import '../../application/sharing_cubit.dart';
import '../../domain/entities/shared_health_package.dart';

/// Page to share health data with another OrionHealth node
class SharePage extends StatelessWidget {
  const SharePage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (_) => SharingCubit()..initialize(),
      child: const _SharePageContent(),
    );
  }
}

class _SharePageContent extends StatefulWidget {
  const _SharePageContent();

  @override
  State<_SharePageContent> createState() => _SharePageContentState();
}

class _SharePageContentState extends State<_SharePageContent> {
  final Set<DataCategory> _selectedCategories = {};
  TransferMethod _selectedMethod = TransferMethod.nfc;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Compartir Datos'),
        actions: [
          IconButton(
            icon: const Icon(Icons.close),
            onPressed: () => Navigator.of(context).pop(),
          ),
        ],
      ),
      body: BlocConsumer<SharingCubit, SharingState>(
        listener: (context, state) {
          if (state is SharingComplete) {
            _showSuccessDialog(context, state.result);
          } else if (state is SharingError) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(state.message), backgroundColor: Colors.red),
            );
          }
        },
        builder: (context, state) {
          if (state is SharingScanning || state is SharingConnecting || state is SharingConnected) {
            return _buildTransferringUI(state);
          }

          return SingleChildScrollView(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _buildCategorySelector(),
                const SizedBox(height: 24),
                _buildMethodSelector(),
                const SizedBox(height: 24),
                _buildShareButton(context, state),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildCategorySelector() {
    return Card(
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
              children: DataCategory.values.map((category) {
                final isSelected = _selectedCategories.contains(category);
                return FilterChip(
                  label: Text(category.displayName),
                  selected: isSelected,
                  onSelected: (selected) {
                    setState(() {
                      if (selected) {
                        _selectedCategories.add(category);
                      } else {
                        _selectedCategories.remove(category);
                      }
                    });
                  },
                );
              }).toList(),
            ),
            const SizedBox(height: 12),
            Text(
              '${_selectedCategories.length} categorías seleccionadas',
              style: const TextStyle(color: Colors.grey),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMethodSelector() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Método de transferencia',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 12),
            ...TransferMethod.values.map((method) {
              return RadioListTile<TransferMethod>(
                title: Text(method.displayName),
                subtitle: Text(method.description),
                value: method,
                groupValue: _selectedMethod,
                onChanged: (value) {
                  if (value != null) {
                    setState(() => _selectedMethod = value);
                  }
                },
              );
            }),
          ],
        ),
      ),
    );
  }

  Widget _buildShareButton(BuildContext context, SharingState state) {
    final canShare = _selectedCategories.isNotEmpty && state is SharingReady;

    return SizedBox(
      width: double.infinity,
      child: ElevatedButton.icon(
        onPressed: canShare ? () => _startSharing(context) : null,
        icon: const Icon(Icons.share),
        label: Text(canShare ? 'Compartir' : 'Selecciona al menos una categoría'),
        style: ElevatedButton.styleFrom(
          padding: const EdgeInsets.all(16),
        ),
      ),
    );
  }

  Widget _buildTransferringUI(SharingState state) {
    String message = 'Transferiendo...';
    double progress = 0.5;

    if (state is SharingScanning) {
      message = 'Buscando dispositivos...';
      progress = 0.2;
    } else if (state is SharingConnecting) {
      message = 'Conectando...';
      progress = 0.4;
    } else if (state is SharingConnected) {
      message = 'Conectado';
      progress = 0.6;
    } else if (state is SharingTransferring) {
      message = state.message;
      progress = state.progress;
    }

    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const CircularProgressIndicator(),
          const SizedBox(height: 24),
          Text(
            message,
            style: const TextStyle(fontSize: 18),
          ),
          const SizedBox(height: 16),
          LinearProgressIndicator(value: progress),
          const SizedBox(height: 32),
          TextButton(
            onPressed: () => context.read<SharingCubit>().cancelSharing(),
            child: const Text('Cancelar'),
          ),
        ],
      ),
    );
  }

  void _startSharing(BuildContext context) {
    // Create package
    final package = SharedHealthPackage(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      senderNodeId: 'my-node-id',
      recipientNodeId: 'target-node-id',
      createdAt: DateTime.now(),
      expiresAt: DateTime.now().add(const Duration(minutes: 3)),
      payload: const EncryptedPayload(
        encryptedData: '',
        iv: '',
        ephemeralPublicKey: '',
      ),
      metadata: PackageMetadata(
        packageType: 'selective',
        consentVerified: true,
        includedCategories: _selectedCategories,
        appVersion: '1.0.0',
      ),
      signature: '',
    );

    context.read<SharingCubit>().startSharing(
      method: _selectedMethod,
      package: package,
    );
  }

  void _showSuccessDialog(BuildContext context, SharingResult result) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        icon: const Icon(Icons.check_circle, color: Colors.green, size: 64),
        title: const Text('¡Compartido exitosamente!'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('${result.bytesTransferred} bytes transferidos'),
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
