// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// A software certificate with a digital signature.
#[derive(Debug, Clone, PartialEq)]
pub struct SignedSoftwareCertificate {
    pub certificate_data: ByteString,
    pub signature: ByteString,
}

impl MessageInfo for SignedSoftwareCertificate {
    fn object_id(&self) -> ObjectId {
        ObjectId::SignedSoftwareCertificate_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SignedSoftwareCertificate> for SignedSoftwareCertificate {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.certificate_data.byte_len();
        size += self.signature.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.certificate_data.encode(stream)?;
        size += self.signature.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let certificate_data = ByteString::decode(stream)?;
        let signature = ByteString::decode(stream)?;
        Ok(SignedSoftwareCertificate {
            certificate_data: certificate_data,
            signature: signature,
        })
    }
}
