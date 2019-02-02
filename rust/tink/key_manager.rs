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

use std::any::TypeId;
use util::*;
use proto::KeyData;

/**
  A KeyManager "understands" keys of a specific key type: it can generate keys of the supported
  type and create primitives for supported keys.
 
  <p>A key type is identified by the global name of the protocol buffer that holds the
  corresponding key material, and is given by {@code typeUrl}-field of {@link KeyData}-protocol
  buffer.
 
  <p>The template parameter {@code P} denotes the primitive corresponding to the keys handled by
  this manager.
 */
pub trait KeyManager<P> {
  /**
    Constructs an instance of P for the key given in {@code serialized_key}, which must be a
    serialized key protocol buffer handled by this manager.
   
    @return the new constructed P
    @throws GeneralSecurityError if the key given in {@code serialized_key} is corrupted or not
        supported
   */
  fn primitive(&self, serialized_key: &[u8]) -> Result<P>;
  
  /**
    Constructs an instance of P for the key given in {@code key}.
   
    @return the new constructed P
    @throws GeneralSecurityException if the key given in {@code key} is corrupted or not supported
   */
  fn primitive(&self, key: MessageLite) -> Result<P>;
  
  /**
    Generates a new key according to specification in {@code serialized_key_format}, which must be a
    serialized key format protocol buffer handled by this manager.
   
    @return the new generated key
    @throws GeneralSecurityException if the specified format is wrong or not supported
   */
  fn new_key(&self, serialized_key_format: &[u8]) -> Result<MessageLite>;
  
  /**
    Generates a new key according to specification in {@code key_format}.
   
    @return the new generated key
    @throws GeneralSecurityException if the specified format is wrong or not supported
   */
  fn new_key(&self, key_format: MessageLite) -> Result<MessageLite>;
  
  /// @return true iff this KeyManager supports key type identified by {@code type_url}.
  fn supports(&self, type_url: String) -> bool;
  
  /// @return the type URL that identifies the key type of keys managed by this KeyManager.
  fn key_type(&self) -> String;
  
  /// @return the version number of this KeyManager.
  fn version(&self) -> u32;
  
  /**
    Returns the primitive {$code TypeId} of the {$code P}.
   */
  fn primitive_type_id(&self) -> TypeId {
    TypeId::of::<P>;
  }
  
  /**
    Generates a new {@code KeyData} according to specification in {@code serialized_key_format}.
   
    <p>This should be used solely by {@link KeysetManager}.
   
    @return the new generated key
    @throws GeneralSecurityError if the specified format is wrong or not supported
   */
  fn new_key_data(&self, serialized_key_format: &[u8]) -> Result<KeyData>;
}
