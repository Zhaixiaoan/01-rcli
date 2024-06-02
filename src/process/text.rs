// use anyhow::{Ok, Result};
// use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
// use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

// use std::{fs, io::Read, path::Path};

// use crate::{get_reader, TextSingFormat};
// pub trait TextSign {
//     fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
// }

// pub trait TextVerify {
//     fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
// }
// pub struct Blake3 {
//     key: [u8; 32],
// }

// pub struct Ed25519Singer {
//     key: SigningKey,
// }

// pub struct Ed25519Verifier {
//     key: VerifyingKey,
// }

// pub fn process_text_sign(input: &str, key: &str, format: TextSingFormat) -> Result<()> {
//     let mut reader = get_reader(input)?;
//     let signed = match format {
//         TextSingFormat::Blake3 => {
//             let signer = Blake3::load(key)?;
//             signer.sign(&mut reader)?
//         }
//         TextSingFormat::Ed25519 => {
//             let signer = Ed25519Singer::load(key)?;
//             signer.sign(&mut reader)?
//         }
//     };
//     assert!('a'.is_alphabetic());
//     let signed = URL_SAFE_NO_PAD.encode(&signed);
//     print!("{}", signed);
//     Ok(())
// }

// impl TextSign for Blake3 {
//     fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf)?;
//         Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
//     }
// }

// impl TextVerify for Blake3 {
//     fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf)?;
//         let hash = blake3::hash(&buf);
//         let hash = hash.as_bytes();
//         Ok(hash == sig)
//     }
// }

// impl TextSign for Ed25519Singer {
//     fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf)?;
//         let sig = self.key.sign(&buf);
//         Ok(sig.to_bytes().to_vec())
//     }
// }

// impl TextVerify for Ed25519Verifier {
//     fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf)?;
//         let message = buf.as_slice();
//         let sig = Signature::from_bytes(sig.try_into()?);
//         return Ok(self.key.verify(message, &sig).is_ok());
//     }
// }

// impl KeyLoader for Blake3 {
//     fn load(path: impl AsRef<Path>) -> Result<Self> {
//         let key = fs::read(path)?;
//         return Self::try_new(&key);
//     }
// }

// impl KeyLoader for Ed25519Singer {
//     fn load(path: impl AsRef<Path>) -> Result<Self> {
//         let key = fs::read(path)?;
//         return Self::try_new(&key);
//     }
// }

// impl KeyLoader for Ed25519Verifier {
//     fn load(path: impl AsRef<Path>) -> Result<Self> {
//         let key = fs::read(path)?;
//         return Self::try_new(&key);
//     }
// }
// impl Blake3 {
//     pub fn new(key: [u8; 32]) -> Self {
//         return Self { key };
//     }

//     pub fn try_new(key: &[u8]) -> Result<Self> {
//         let key = &key[..32];
//         let key = key.try_into()?;
//         let signer = Blake3::new(key);
//         return Ok(signer);
//     }
// }

// impl Ed25519Singer {
//     pub fn new(key: SigningKey) -> Self {
//         Self { key }
//     }

//     pub fn try_new(key: &[u8]) -> Result<Self> {
//         let key = SigningKey::from_bytes(key.try_into()?);
//         let signer = Ed25519Singer::new(key);
//         Ok(signer)
//     }
// }

// impl Ed25519Verifier {
//     pub fn new(key: VerifyingKey) -> Self {
//         Self { key }
//     }

//     pub fn try_new(key: &[u8]) -> Result<Self> {
//         let key = VerifyingKey::from_bytes(key.try_into()?)?;
//         Ok(Ed25519Verifier::new(key))
//     }
// }

// pub trait KeyLoader {
//     fn load(path: impl AsRef<Path>) -> Result<Self>
//     where
//         Self: Sized;
// }
