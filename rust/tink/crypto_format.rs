// Copyright 2017 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

mod util;
mod proto;

use std::vec;
use util::Result;
use proto::Keyset::Key;

/// Prefix size of Tink and Legacy key types.
pub const NON_RAW_PREFIX_SIZE = 5u64;

/// Legacy or Crunchy prefix starts with \x00 and followed by a 4-byte key id.
pub const LEGACY_PREFIX_SIZE = NON_RAW_PREFIX_SIZE;

pub const LEGACY_START_BYTE = 0u8;

/// Tink prefix starts with \x01 and followed by a 4-byte key id.
pub const TINK_PREFIX_SIZE = NON_RAW_PREFIX_SIZE;

pub const TINK_START_BYTE = 1u8;

/// Raw prefix is empty.
pub const RAW_PREFIX_SIZE = 0u64;

pub const RAW_PREFIX = [u8; 0] = [];

pub fn output_prefix(key: Key) -> Result<&[u8]> {
  match key.output_prefix_type {
    Key::LEGACY | Key::CRUNCHY => create_output_prefix(LEGACY_PREFIX_SIZE, LEGACY_START_BYTE, key.KeyId),
    Key::TINK => create_output_prefix(TINK_PREFIX_SIZE, TINK_START_BYTE, key.KeyId),
    Key::RAW => Ok(RAW_PREFIX),
    _ => GeneralSecurityError("unknown output prefix type")
  }
}

fn create_output_prefix(size: u64, start_byte: u8, keyId: u32) -> &[u8] {
  let mut prefix = vec![0u8; size];
  prefix[0] = start_byte;
  prefix.write(keyId);
  &prefix;
}
