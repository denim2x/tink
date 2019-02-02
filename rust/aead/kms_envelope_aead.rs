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

mod tink;
mod util;

use tink::Aead;
use tink::Registry;
use proto::KeyTemplate;
use util::*;

/**
  This primitive implements <a href="https://cloud.google.com/kms/docs/data-encryption-keys">
  envelope encryption</a>. In envelope encryption, user generates a data encryption key (DEK)
  locally, encrypts data with DEK, sends DEK to a KMS to be encrypted (with a key managed by KMS),
  and stores encrypted DEK with encrypted data; at a later point user can retrieve encrypted data
  and DEK, use Storky to decrypt DEK, and use decrypted DEK to decrypt the data.
  The ciphertext structure is as follows:
    - Length of encrypted DEK: 4 bytes.
    - Encrypted DEK: variable length that is equal to the value specified in the last 4 bytes.
    - AEAD payload: variable length.
 */
pub struct KmsEnvelopeAead {
  
}

impl Aead for KmsEnvelopeAead {
  
}
