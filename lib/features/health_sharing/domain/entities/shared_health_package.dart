import 'dart:convert';
import 'package:equatable/equatable.dart';

/// Represents an encrypted health data package for P2P sharing
class SharedHealthPackage extends Equatable {
  final String id;
  final String senderNodeId;
  final String recipientNodeId;
  final DateTime createdAt;
  final DateTime expiresAt;
  final EncryptedPayload payload;
  final PackageMetadata metadata;
  final String signature;

  const SharedHealthPackage({
    required this.id,
    required this.senderNodeId,
    required this.recipientNodeId,
    required this.createdAt,
    required this.expiresAt,
    required this.payload,
    required this.metadata,
    required this.signature,
  });

  bool get isExpired => DateTime.now().isAfter(expiresAt);

  Duration get timeRemaining {
    final remaining = expiresAt.difference(DateTime.now());
    return remaining.isNegative ? Duration.zero : remaining;
  }

  bool get canShare => !isExpired && metadata.consentVerified;

  Map<String, dynamic> toJson() => {
        'id': id,
        'senderNodeId': senderNodeId,
        'recipientNodeId': recipientNodeId,
        'createdAt': createdAt.toIso8601String(),
        'expiresAt': expiresAt.toIso8601String(),
        'payload': payload.toJson(),
        'metadata': metadata.toJson(),
        'signature': signature,
      };

  factory SharedHealthPackage.fromJson(Map<String, dynamic> json) {
    return SharedHealthPackage(
      id: json['id'],
      senderNodeId: json['senderNodeId'],
      recipientNodeId: json['recipientNodeId'],
      createdAt: DateTime.parse(json['createdAt']),
      expiresAt: DateTime.parse(json['expiresAt']),
      payload: EncryptedPayload.fromJson(json['payload']),
      metadata: PackageMetadata.fromJson(json['metadata']),
      signature: json['signature'],
    );
  }

  String encode() => base64Encode(utf8.encode(jsonEncode(toJson())));

  static SharedHealthPackage decode(String encoded) {
    return SharedHealthPackage.fromJson(jsonDecode(utf8.decode(base64Decode(encoded))));
  }

  @override
  List<Object?> get props => [id, senderNodeId, recipientNodeId, createdAt, expiresAt];
}

/// Encrypted payload containing health data
class EncryptedPayload extends Equatable {
  final String encryptedData; // AES-256-GCM encrypted JSON
  final String iv; // Initialization vector
  final String ephemeralPublicKey; // ECDH ephemeral key for recipient

  const EncryptedPayload({
    required this.encryptedData,
    required this.iv,
    required this.ephemeralPublicKey,
  });

  Map<String, dynamic> toJson() => {
        'encryptedData': encryptedData,
        'iv': iv,
        'ephemeralPublicKey': ephemeralPublicKey,
      };

  factory EncryptedPayload.fromJson(Map<String, dynamic> json) {
    return EncryptedPayload(
      encryptedData: json['encryptedData'],
      iv: json['iv'],
      ephemeralPublicKey: json['ephemeralPublicKey'],
    );
  }

  @override
  List<Object?> get props => [encryptedData, iv, ephemeralPublicKey];
}

/// Metadata about the shared package
class PackageMetadata extends Equatable {
  final String packageType; // 'full' or 'selective'
  final bool consentVerified;
  final Set<DataCategory> includedCategories;
  final String? pinHash; // Hash of PIN for verification
  final String appVersion;

  const PackageMetadata({
    required this.packageType,
    required this.consentVerified,
    required this.includedCategories,
    this.pinHash,
    required this.appVersion,
  });

  Map<String, dynamic> toJson() => {
        'packageType': packageType,
        'consentVerified': consentVerified,
        'includedCategories': includedCategories.map((c) => c.name).toList(),
        'pinHash': pinHash,
        'appVersion': appVersion,
      };

  factory PackageMetadata.fromJson(Map<String, dynamic> json) {
    return PackageMetadata(
      packageType: json['packageType'],
      consentVerified: json['consentVerified'],
      includedCategories: (json['includedCategories'] as List)
          .map((name) => DataCategory.valueOf(name))
          .toSet(),
      pinHash: json['pinHash'],
      appVersion: json['appVersion'],
    );
  }

  @override
  List<Object?> get props => [packageType, consentVerified, includedCategories, appVersion];
}

/// Categories of health data that can be shared
enum DataCategory {
  labResults('Laboratorios'),
  vitalSigns('Signos Vitales'),
  medications('Medicamentos'),
  medicalEvents('Eventos Médicos'),
  documents('Documentos'),
  allergies('Alergias'),
  conditions('Condiciones'),
  procedures('Procedimientos');

  final String displayName;
  const DataCategory(this.displayName);

  static DataCategory valueOf(String name) {
    return DataCategory.values.firstWhere(
      (c) => c.name == name,
      orElse: () => DataCategory.labResults,
    );
  }
}

/// Result of a sharing operation
class SharingResult extends Equatable {
  final bool success;
  final String? error;
  final int bytesTransferred;
  final Duration transferTime;

  const SharingResult({
    required this.success,
    this.error,
    required this.bytesTransferred,
    required this.transferTime,
  });

  @override
  List<Object?> get props => [success, error, bytesTransferred, transferTime];
}

/// Transfer method options
enum TransferMethod {
  nfc('NFC', 'Tap phones to share'),
  ble('Bluetooth', 'Nearby device'),
  wifi('WiFi Direct', 'Same network');

  final String displayName;
  final String description;
  const TransferMethod(this.displayName, this.description);
}
