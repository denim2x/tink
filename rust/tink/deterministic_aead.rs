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
use util::Result;

pub trait DeterministicAead {
  /**
    Deterministically encrypts {@code plaintext} with {@code associated_data} as associated
    authenticated data.
   
    <p><b>Warning</b>
   
    <p>Encrypting the same {@code plaintext} multiple times protects the integrity of that
    plaintext, but confidentiality is compromised to the extent that an attacker can determine that
    the same plaintext was encrypted.
   
    <p>The resulting ciphertext allows for checking authenticity and integrity of associated data
    ({@code associated_data}), but does not guarantee its secrecy.
   
    @return resulting ciphertext
   */
  fn encrypt_deterministically(&self, plaintext: &[u8], associated_data: &[u8]) -> Result<&[u8]>;
  
  /**
    Deterministically decrypts {@code ciphertext} with {@code associated_data} as associated
    authenticated data.
   
    <p>The decryption verifies the authenticity and integrity of the associated data, but there are
    no guarantees wrt. secrecy of that data.
   
    @return resulting plaintext
   */
  fn decrypt_deterministically(&self, ciphertext: &[u8], associated_data: &[u8]) -> Result<&[u8]>;
}
