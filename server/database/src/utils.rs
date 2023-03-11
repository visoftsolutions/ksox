use bytes::{Bytes, BytesMut};
use sha2::{Digest, Sha256};

pub fn trigger_name(prefix: &str, vec: Vec<Bytes>) -> String {
    let mut hasher = Sha256::new();
    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(prefix.as_bytes());
    bytes.extend(vec.into_iter());
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bytes::Bytes;
    use uuid::Uuid;

    use super::trigger_name;

    #[test]
    fn test_trigger_name() {
        let uuid1 = Uuid::from_str("06a5c021-0dcd-4e06-892c-26b1a573b965").unwrap();
        let uuid2 = Uuid::from_str("1a68ba48-92c5-499f-bc0e-a7f73fd619ce").unwrap();

        assert_eq!(
            trigger_name(
                "abc",
                vec![
                    Bytes::from(uuid1.as_bytes().to_owned().to_vec()),
                    Bytes::from(uuid2.as_bytes().to_owned().to_vec())
                ],
            ),
            "e770309ae99fab903410d4c6de8772fad351ce48263fd16df51ffb68c7e4cbba".to_owned()
        );
    }
}
