import 'dart:convert';
import 'dart:async';
import 'package:flutter/services.dart';
import '../domain/entities/shared_health_package.dart';

/// NFC sharing service for tap-to-share health data
class NfcSharingService {
  static const MethodChannel _channel = MethodChannel('orionhealth/nfc');

  bool _isInitialized = false;
  bool _isEnabled = false;

  final _stateController = StreamController<NfcSharingState>.broadcast();
  Stream<NfcSharingState> get stateStream => _stateController.stream;

  final _dataController = StreamController<SharedHealthPackage>.broadcast();
  Stream<SharedHealthPackage> get incomingData => _dataController.stream;

  /// Initialize NFC adapter
  Future<void> initialize() async {
    if (_isInitialized) return;

    try {
      final result = await _channel.invokeMethod<bool>('isNfcAvailable');
      _isEnabled = result ?? false;

      if (_isEnabled) {
        // Start NFC session in native code
        await _channel.invokeMethod('startNfcSession');
      }

      _isInitialized = true;
      _stateController.add(NfcSharingState.ready(isEnabled: _isEnabled));
    } on PlatformException catch (e) {
      _stateController.add(NfcSharingState.error('NFC not available: ${e.message}'));
      _isEnabled = false;
      _isInitialized = true;
    }
  }

  /// Check if NFC is available and enabled
  Future<bool> isAvailable() async {
    if (!_isInitialized) await initialize();
    return _isEnabled;
  }

  /// Share data via NFC beam
  Future<SharingResult> shareData(SharedHealthPackage package) async {
    if (!_isEnabled) {
      return SharingResult(
        success: false,
        error: 'NFC not available',
        bytesTransferred: 0,
        transferTime: Duration.zero,
      );
    }

    _stateController.add(NfcSharingState.ndefBeam(
      package.recipientNodeId,
      'Sharing ${package.metadata.includedCategories.length} categories...',
    ));

    final startTime = DateTime.now();

    try {
      final data = package.encode();

      // In production, use Android Beam or iOS NFC:
      // await _channel.invokeMethod('beamNdefMessage', {'data': data});

      await Future.delayed(const Duration(seconds: 1)); // Simulate beam

      final transferTime = DateTime.now().difference(startTime);

      _stateController.add(NfcSharingState.completed(
        data.length,
        transferTime,
      ));

      return SharingResult(
        success: true,
        bytesTransferred: data.length,
        transferTime: transferTime,
      );
    } on PlatformException catch (e) {
      _stateController.add(NfcSharingState.error('NFC share failed: ${e.message}'));
      return SharingResult(
        success: false,
        error: e.message,
        bytesTransferred: 0,
        transferTime: DateTime.now().difference(startTime),
      );
    }
  }

  /// Start listening for incoming NFC data
  Future<void> startListening() async {
    if (!_isEnabled) return;

    _stateController.add(NfcSharingState.listening());

    // In production, this would be handled by native code
    // and trigger onNdefDataReceived callback
  }

  /// Stop listening for NFC
  Future<void> stopListening() async {
    await _channel.invokeMethod('stopNfcSession');
    _stateController.add(NfcSharingState.ready(isEnabled: _isEnabled));
  }

  /// Handle received NFC data (called from native side)
  void handleReceivedData(String encodedPackage) {
    try {
      final package = SharedHealthPackage.decode(encodedPackage);

      if (package.isExpired) {
        _stateController.add(NfcSharingState.error('Package has expired'));
        return;
      }

      _dataController.add(package);
      _stateController.add(NfcSharingState.received(package));
    } catch (e) {
      _stateController.add(NfcSharingState.error('Failed to parse package: $e'));
    }
  }

  /// Clean up resources
  void dispose() {
    stopListening();
    _stateController.close();
    _dataController.close();
  }
}

/// State of NFC sharing service
class NfcSharingState {
  final String status;
  final String? peerId;
  final String? message;
  final bool isError;
  final bool isEnabled;
  final int? bytesTransferred;
  final Duration? transferTime;
  final SharedHealthPackage? receivedPackage;

  const NfcSharingState._({
    required this.status,
    this.peerId,
    this.message,
    this.isError = false,
    this.isEnabled = true,
    this.bytesTransferred,
    this.transferTime,
    this.receivedPackage,
  });

  factory NfcSharingState.ready({bool isEnabled = true}) => NfcSharingState._(
        status: 'ready',
        isEnabled: isEnabled,
        message: isEnabled ? 'Ready to share via NFC' : 'NFC not available',
      );

  factory NfcSharingState.listening() => const NfcSharingState._(
        status: 'listening',
        message: 'Tap another OrionHealth device to share...',
      );

  factory NfcSharingState.ndefBeam(String peerId, String message) => NfcSharingState._(
        status: 'ndef_beam',
        peerId: peerId,
        message: message,
      );

  factory NfcSharingState.received(SharedHealthPackage package) => NfcSharingState._(
        status: 'received',
        message: 'Data received from ${package.senderNodeId}',
        receivedPackage: package,
      );

  factory NfcSharingState.completed(int bytes, Duration time) => NfcSharingState._(
        status: 'completed',
        message: 'Transfer complete',
        bytesTransferred: bytes,
        transferTime: time,
      );

  factory NfcSharingState.error(String message) => NfcSharingState._(
        status: 'error',
        message: message,
        isError: true,
      );
}