use ethers_core::types::Signature;
use rand::RngCore;

pub fn random_bytes(size: usize) -> Vec<u8> {
    let mut vec = Vec::<u8>::with_capacity(size);
    rand::thread_rng().fill_bytes(vec.as_mut_slice());
    vec
}

pub fn check_signature(
    address: &str,
    message: &[u8],
    signature: &str,
) -> Result<(), anyhow::Error> {
    let signature = Signature::try_from(
        prefix_hex::decode::<Vec<u8>>(signature.to_lowercase().as_str())
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .as_slice(),
    )?;
    let address = prefix_hex::decode::<[u8; 20]>(address.to_lowercase().as_str())
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    signature.verify(message, address)?;
    Ok(())
}

pub fn recover_author(message: &[u8], signature: &str) -> Result<String, anyhow::Error> {
    Ok(format!(
        "{:02X?}",
        Signature::try_from(
            prefix_hex::decode::<Vec<u8>>(signature.to_lowercase().as_str())
                .map_err(|e| anyhow::Error::msg(e.to_string()))?
                .as_slice()
        )?
        .recover(message)?
    ))
}

#[test]
fn test_check_signature() {
    let account = "0xC0dbFCb6f274fA7974638Ab9fA5040decFD06c75";
    let message = "Hello World".as_bytes();
    let signature ="0x68becbe4b7101ed2f1542d20b9eb5c42d28839d3927d314d1c34b866fd2acffa7e828005d34ed6010bb8fb4d5c4340cb7a9108d6e6971b90fae4295d1924cb821b";
    assert!(check_signature(account, message, signature).is_ok());
}

#[test]
fn test_recover_author() {
    let account = "0xC0dbFCb6f274fA7974638Ab9fA5040decFD06c75";
    let message = "Hello World".as_bytes();
    let signature ="0x68becbe4b7101ed2f1542d20b9eb5c42d28839d3927d314d1c34b866fd2acffa7e828005d34ed6010bb8fb4d5c4340cb7a9108d6e6971b90fae4295d1924cb821b";
    let address = recover_author(message, signature).unwrap();
    assert_eq!(account.to_lowercase(), address);
}
