use openssl::hash::{Hasher, MessageDigest};
use std::io::Read;

pub enum Digest {
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
}

impl Into<MessageDigest> for Digest {
    fn into(self) -> MessageDigest {
        match self {
            Digest::MD5 => MessageDigest::md5(),
            Digest::SHA1 => MessageDigest::sha1(),
            Digest::SHA224 => MessageDigest::sha224(),
            Digest::SHA256 => MessageDigest::sha256(),
            Digest::SHA384 => MessageDigest::sha384(),
            Digest::SHA512 => MessageDigest::sha512(),
        }
    }
}

pub fn data_hash(data: &[u8], mode: Digest) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let digest_bytes = openssl::hash::hash(mode.into(), data);
    digest_bytes
        .map(|t| t.as_ref().to_owned())
        .map_err(|e| e.into())
}

pub fn chunk_hash(
    mut input: impl Read,
    mode: Digest,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut h = Hasher::new(mode.into()).map_err(|e| Box::new(e))?;
    let mut buff: [u8; 4096] = [0; 4096];
    let mut n_read = 0;
    loop {
        n_read = input.read(&mut buff)?;
        if n_read == 0 {
            break;
        }
        h.update(&buff[..n_read])?;
    }
    h.finish()
        .map(|t| t.as_ref().to_owned())
        .map_err(|e| e.into())
}

#[cfg(test)]
mod test {
    use crate::crypto::hash;
    use crate::crypto::hash::Digest;
    use crate::crypto::hex::HexDisplay;
    #[test]
    fn test_hash() {
        let data = b"Hello,World! Nice to meet you!";
        let result = "fc6dd3d81fca22163558f69bb9e4fbc3";
        let hash = hash::data_hash(data.as_slice(), Digest::MD5).unwrap();
        let hash_str = hash.hex_str();
        assert_eq!(result, hash_str);
    }
}
