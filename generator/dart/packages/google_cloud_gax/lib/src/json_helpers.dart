// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Helper utility methods for JSON encoding and decoding from [Message]
/// objects.
library;

import 'dart:convert';
import 'dart:typed_data';

import '../common.dart';

T? decode<T, S>(dynamic json, T Function(S) decoder) {
  return json == null ? null : decoder(json);
}

List<T>? decodeList<T, S>(dynamic json, T Function(S) decoder) {
  return (json as List?)?.map((item) => decoder(item)).toList().cast();
}

Map<String, T>? decodeMap<T>(
    dynamic json, T Function(Map<String, dynamic>) decoder) {
  return (json as Map?)
      ?.map((key, value) => MapEntry(key, decoder(value)))
      .cast();
}

List? encodeList(List<JsonEncodable>? items) {
  return items?.map((item) => item.toJson()).toList();
}

Map? encodeMap(Map<String, JsonEncodable>? items) {
  return items?.map((key, value) => MapEntry(key, value.toJson()));
}

/// Decode a base64 encoded bytes value into a [Uint8List].
Uint8List? decodeBytes(String? value) =>
    value == null ? null : base64Decode(value);

/// Encode a [Uint8List] into a base64 encoded bytes representation.
String encodeBytes(Uint8List data) => base64Encode(data);
