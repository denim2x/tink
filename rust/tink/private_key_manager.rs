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

use util::Result;
use proto::KeyData;

/// A {@link KeyManager} that understands asymmetric private key types.
pub trait PrivateKeyManager<P> {
  /**
    Extracts the public key data from the private key data.
   
    @return the {@link KeyData} containing the public keys
    @throws GeneralSecurityError if the specified format is wrong or not supported
   */
  fn public_key_data(&self, serialized_key: &[u8]) -> Result<KeyData>;
}
