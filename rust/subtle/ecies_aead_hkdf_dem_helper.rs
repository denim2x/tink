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
use util::*;

/// A helper for DEM (data encapsulation mechanism) of ECIES-AEAD-HKDF.
pub trait EciesAeadHkdfDemHelper {
  /// @return the size of the DEM-key in bytes.
  fn symmetric_key_sz_bytes(&self) -> u32;
  
  /**
    Creates a new {@code Aead}-primitive that uses the key material given in 'symmetric_key', which
    must be of length dem_key_size_in_bytes().
   
    @return the newly created {@code Aead}-primitive.
   */
  fn aead(&self, symmetric_key: &[u8]) -> Result<Aead>;
}
