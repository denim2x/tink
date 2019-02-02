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
use util::*;

/// A KmsClient knows how to produce primitives backed by keys stored in remote KMS services.
pub trait KmsClient {
  /// @return true if this client does support {@code key_uri}
  fn supports(&self, key_uri: String) -> bool;
  
  /**
    Loads the credentials in {@code credential_path}. If {@code credential_path} is null, loads the
    default credentials.
   */
  fn with_credentials(&self, credential_path: String) -> Result<KmsClient>;
  
  /// Loads the default credentials.
  fn with_default_credentials(&self, credential_path: String) -> Result<KmsClient>;
  
  /**
    Gets an {@code Aead} backed by {@code key_uri}.
   
    @throws GeneralSecurityError if the URI is not supported or invalid
   */
  fn aead(&self, key: String) -> Result<Aead>;
}
