// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// A token representing a user identified by a user name and password.
#[derive(Debug, Clone, PartialEq)]
pub struct UserNameIdentityToken {
    pub policy_id: UAString,
    pub user_name: UAString,
    pub password: ByteString,
    pub encryption_algorithm: UAString,
}

impl BinaryEncoder<UserNameIdentityToken> for UserNameIdentityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.policy_id.byte_len();
        size += self.user_name.byte_len();
        size += self.password.byte_len();
        size += self.encryption_algorithm.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.policy_id.encode(stream)?;
        size += self.user_name.encode(stream)?;
        size += self.password.encode(stream)?;
        size += self.encryption_algorithm.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let policy_id = UAString::decode(stream)?;
        let user_name = UAString::decode(stream)?;
        let password = ByteString::decode(stream)?;
        let encryption_algorithm = UAString::decode(stream)?;
        Ok(UserNameIdentityToken {
            policy_id: policy_id,
            user_name: user_name,
            password: password,
            encryption_algorithm: encryption_algorithm,
        })
    }
}
