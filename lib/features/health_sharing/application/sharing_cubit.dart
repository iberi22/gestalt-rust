import 'dart:async';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:equatable/equatable.dart';
import '../domain/entities/shared_health_package.dart';
import '../infrastructure/ble_sharing_service.dart';
import '../infrastructure/nfc_sharing_service.dart';
import '../infrastructure/wifi_direct_service.dart';

// ============================================================================
// STATES
// ============================================================================

abstract class SharingState extends Equatable {
  const SharingState();

  @override
  List<Object?> get props => [];
}

class SharingInitial extends SharingState {}

class SharingReady extends SharingState {}

class SharingScanning extends SharingState {
  final TransferMethod method;
  const SharingScanning(this.method);

  @override
  List<Object?> get props => [method];
}

class SharingAdvertising extends SharingState {
  final TransferMethod method;
  final String nodeId;

  const SharingAdvertising(this.method, this.nodeId);

  @override
  List<Object?> get props => [method, nodeId];
}

class SharingConnecting extends SharingState {
  final TransferMethod method;
  final String deviceId;

  const SharingConnecting(this.method, this.deviceId);

  @override
  List<Object?> get props => [method, deviceId];
}

class SharingConnected extends SharingState {
  final TransferMethod method;
  final String deviceId;

  const SharingConnected(this.method, this.deviceId);

  @override
  List<Object?> get props => [method, deviceId];
}

class SharingTransferring extends SharingState {
  final TransferMethod method;
  final double progress;
  final String message;

  const SharingTransferring({
    required this.method,
    required this.progress,
    required this.message,
  });

  @override
  List<Object?> get props => [method, progress, message];
}

class SharingComplete extends SharingState {
  final SharingResult result;
  final TransferMethod method;

  const SharingComplete(this.result, this.method);

  @override
  List<Object?> get props => [result, method];
}

class SharingReceiving extends SharingState {
  final SharedHealthPackage? package;
  final TransferMethod method;

  const SharingReceiving({this.package, required this.method});

  @override
  List<Object?> get props => [package, method];
}

class SharingError extends SharingState {
  final String message;
  const SharingError(this.message);

  @override
  List<Object?> get props => [message];
}

// ============================================================================
// CUBIT
// ============================================================================

class SharingCubit extends Cubit<SharingState> {
  final BleSharingService _bleService;
  final NfcSharingService _nfcService;
  final WifiDirectService _wifiService;

  StreamSubscription? _bleSubscription;
  StreamSubscription? _nfcSubscription;
  StreamSubscription? _wifiSubscription;

  TransferMethod? _currentMethod;
  SharedHealthPackage? _pendingPackage;

  SharingCubit({
    BleSharingService? bleService,
    NfcSharingService? nfcService,
    WifiDirectService? wifiService,
  })  : _bleService = bleService ?? BleSharingService(),
        _nfcService = nfcService ?? NfcSharingService(),
        _wifiService = wifiService ?? WifiDirectService(),
        super(SharingInitial());

  /// Initialize all services
  Future<void> initialize() async {
    await _bleService.initialize();
    await _nfcService.initialize();
    await _wifiService.initialize();

    _setupSubscriptions();

    emit(SharingReady());
  }

  void _setupSubscriptions() {
    _bleSubscription = _bleService.stateStream.listen((state) {
      _handleBleState(state);
    });

    _nfcSubscription = _nfcService.stateStream.listen((state) {
      _handleNfcState(state);
    });

    _wifiSubscription = _wifiService.stateStream.listen((state) {
      _handleWifiState(state);
    });
  }

  void _handleBleState(BleSharingState state) {
    if (state.status == 'scanning') {
      emit(SharingScanning(TransferMethod.ble));
    } else if (state.status == 'advertising') {
      emit(SharingAdvertising(TransferMethod.ble, state.deviceId ?? ''));
    } else if (state.status == 'connecting') {
      emit(SharingConnecting(TransferMethod.ble, state.deviceId ?? ''));
    } else if (state.status == 'connected') {
      emit(SharingConnected(TransferMethod.ble, state.deviceId ?? ''));
    } else if (state.status == 'transferring') {
      emit(SharingTransferring(
        method: TransferMethod.ble,
        progress: 0.5,
        message: state.message ?? 'Transferring...',
      ));
    } else if (state.status == 'completed') {
      emit(SharingComplete(
        SharingResult(
          success: true,
          bytesTransferred: state.bytesTransferred ?? 0,
          transferTime: state.transferTime ?? Duration.zero,
        ),
        TransferMethod.ble,
      ));
    } else if (state.isError) {
      emit(SharingError(state.message ?? 'BLE Error'));
    }
  }

  void _handleNfcState(NfcSharingState state) {
    if (state.status == 'listening') {
      emit(const SharingScanning(TransferMethod.nfc));
    } else if (state.status == 'ndef_beam') {
      emit(SharingTransferring(
        method: TransferMethod.nfc,
        progress: 0.5,
        message: state.message ?? 'Beaming...',
      ));
    } else if (state.status == 'received') {
      emit(SharingReceiving(
        package: state.receivedPackage,
        method: TransferMethod.nfc,
      ));
    } else if (state.status == 'completed') {
      emit(SharingComplete(
        SharingResult(
          success: true,
          bytesTransferred: state.bytesTransferred ?? 0,
          transferTime: state.transferTime ?? Duration.zero,
        ),
        TransferMethod.nfc,
      ));
    } else if (state.isError) {
      emit(SharingError(state.message ?? 'NFC Error'));
    }
  }

  void _handleWifiState(WifiSharingState state) {
    if (state.status == 'discovering') {
      emit(SharingScanning(TransferMethod.wifi));
    } else if (state.status == 'hosting') {
      emit(SharingAdvertising(TransferMethod.wifi, state.address ?? ''));
    } else if (state.status == 'connecting') {
      emit(SharingConnecting(TransferMethod.wifi, state.address ?? ''));
    } else if (state.status == 'transferring') {
      emit(SharingTransferring(
        method: TransferMethod.wifi,
        progress: 0.5,
        message: state.message ?? 'Transferring...',
      ));
    } else if (state.status == 'received') {
      emit(SharingReceiving(
        package: state.receivedPackage,
        method: TransferMethod.wifi,
      ));
    } else if (state.status == 'completed') {
      emit(SharingComplete(
        SharingResult(
          success: true,
          bytesTransferred: state.bytesTransferred ?? 0,
          transferTime: state.transferTime ?? Duration.zero,
        ),
        TransferMethod.wifi,
      ));
    } else if (state.isError) {
      emit(SharingError(state.message ?? 'WiFi Error'));
    }
  }

  // ==========================================================================
  // SEND DATA
  // ==========================================================================

  /// Start sharing data via selected method
  Future<void> startSharing({
    required TransferMethod method,
    required SharedHealthPackage package,
  }) async {
    _pendingPackage = package;
    _currentMethod = method;

    switch (method) {
      case TransferMethod.ble:
        await _bleService.startAdvertising(package.recipientNodeId);
        break;
      case TransferMethod.nfc:
        await _nfcService.shareData(package);
        break;
      case TransferMethod.wifi:
        await _wifiService.startServer();
        break;
    }
  }

  /// Send via BLE
  Future<void> sendViaBle(String deviceId, SharedHealthPackage package) async {
    final connected = await _bleService.connect(deviceId);
    if (!connected) {
      emit(const SharingError('Failed to connect'));
      return;
    }

    final result = await _bleService.sendData(package);
    if (result.success) {
      emit(SharingComplete(result, TransferMethod.ble));
    } else {
      emit(SharingError(result.error ?? 'Transfer failed'));
    }
  }

  /// Send via WiFi Direct
  Future<void> sendViaWifi(String targetIp, SharedHealthPackage package) async {
    final result = await _wifiService.sendData(targetIp, package);
    if (result.success) {
      emit(SharingComplete(result, TransferMethod.wifi));
    } else {
      emit(SharingError(result.error ?? 'Transfer failed'));
    }
  }

  /// Cancel current sharing
  Future<void> cancelSharing() async {
    await _bleService.disconnect();
    await _bleService.stopAdvertising();
    await _nfcService.stopListening();
    await _wifiService.stop();

    _pendingPackage = null;
    _currentMethod = null;

    emit(SharingReady());
  }

  // ==========================================================================
  // RECEIVE DATA
  // ==========================================================================

  /// Start listening for incoming data
  Future<void> startListening(TransferMethod method) async {
    _currentMethod = method;

    switch (method) {
      case TransferMethod.ble:
        await _bleService.scanForDevices();
        break;
      case TransferMethod.nfc:
        await _nfcService.startListening();
        break;
      case TransferMethod.wifi:
        await _wifiService.startServer();
        break;
    }
  }

  /// Handle incoming package
  void handleIncomingPackage(SharedHealthPackage package) {
    emit(SharingReceiving(package: package, method: _currentMethod!));
  }

  /// Accept and import incoming package
  Future<void> acceptIncomingPackage() async {
    // Import to wallet
    // TODO: Integrate with HealthWalletService
    emit(SharingReady());
  }

  /// Reject incoming package
  void rejectIncomingPackage() {
    emit(SharingReady());
  }

  // ==========================================================================
  // DISCOVERY
  // ==========================================================================

  /// Scan for BLE devices
  Future<List<BleDevice>> scanBleDevices() async {
    return await _bleService.scanForDevices();
  }

  /// Discover WiFi devices
  Future<List<WifiDirectDevice>> discoverWifiDevices() async {
    return await _wifiService.discoverDevices();
  }

  // ==========================================================================
  // UTILITIES
  // ==========================================================================

  /// Reset to ready state
  void reset() {
    emit(SharingReady());
  }

  @override
  Future<void> close() async {
    await _bleSubscription?.cancel();
    await _nfcSubscription?.cancel();
    await _wifiSubscription?.cancel();

    _bleService.dispose();
    _nfcService.dispose();
    _wifiService.dispose();

    return super.close();
  }
}
