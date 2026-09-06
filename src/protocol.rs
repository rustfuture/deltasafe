use std::io::{Read, Write};
use std::net::TcpStream;

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, bail, Context, Result};
use hkdf::Hkdf;
use sha2::Sha256;

use crate::sync::CHUNK_SIZE;

pub(crate) const PROTOCOL_VERSION: u16 = 1;
pub(crate) const MAX_HEADER_SIZE: usize = 64 * 1024;
pub(crate) const MAX_FILE_SIZE: u64 = 8 * 1024 * 1024 * 1024;

pub(crate) const KIND_DATA: u8 = 1;
pub(crate) const KIND_FINISH: u8 = 2;
pub(crate) const KIND_READY: u8 = 3;
pub(crate) const KIND_COMPLETE: u8 = 4;
pub(crate) const KIND_ERROR: u8 = 5;

pub(crate) const CLIENT_DIRECTION: [u8; 4] = *b"C2S\0";
pub(crate) const SERVER_DIRECTION: [u8; 4] = *b"S2C\0";

const FRAME_METADATA_SIZE: usize = 1 + std::mem::size_of::<u64>();
const GCM_TAG_SIZE: usize = 16;
const MAX_FRAME_SIZE: usize = FRAME_METADATA_SIZE + CHUNK_SIZE + GCM_TAG_SIZE;

#[derive(Debug)]
pub(crate) struct EncryptedFrame {
    pub kind: u8,
    pub index: u64,
    pub ciphertext: Vec<u8>,
}

pub(crate) fn derive_session_key(base_key: &[u8; 32], session_id: &[u8; 16]) -> Result<[u8; 32]> {
    let hkdf = Hkdf::<Sha256>::new(Some(session_id), base_key);
    let mut session_key = [0u8; 32];
    hkdf.expand(b"deltasafe/protocol/v1/session-key", &mut session_key)
        .map_err(|_| anyhow!("Oturum anahtarı türetilemedi"))?;
    Ok(session_key)
}

fn nonce(direction: [u8; 4], index: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[..4].copy_from_slice(&direction);
    nonce[4..].copy_from_slice(&index.to_be_bytes());
    nonce
}

fn additional_data(header_bytes: &[u8], direction: [u8; 4], kind: u8, index: u64) -> Vec<u8> {
    let mut aad = Vec::with_capacity(header_bytes.len() + 13);
    aad.extend_from_slice(header_bytes);
    aad.extend_from_slice(&direction);
    aad.push(kind);
    aad.extend_from_slice(&index.to_be_bytes());
    aad
}

pub(crate) fn encrypt_frame(
    header_bytes: &[u8],
    session_key: &[u8; 32],
    direction: [u8; 4],
    kind: u8,
    index: u64,
    plaintext: &[u8],
) -> Result<Vec<u8>> {
    if plaintext.len() > CHUNK_SIZE {
        bail!("Frame verisi sınırı aşıyor: {} bayt", plaintext.len());
    }

    let cipher =
        Aes256Gcm::new_from_slice(session_key).map_err(|_| anyhow!("Geçersiz oturum anahtarı"))?;
    let nonce_bytes = nonce(direction, index);
    let aad = additional_data(header_bytes, direction, kind, index);
    let ciphertext = cipher
        .encrypt(
            Nonce::from_slice(&nonce_bytes),
            Payload {
                msg: plaintext,
                aad: &aad,
            },
        )
        .map_err(|_| anyhow!("Frame şifrelenemedi"))?;

    let mut body = Vec::with_capacity(FRAME_METADATA_SIZE + ciphertext.len());
    body.push(kind);
    body.extend_from_slice(&index.to_be_bytes());
    body.extend_from_slice(&ciphertext);
    Ok(body)
}

pub(crate) fn decrypt_frame(
    header_bytes: &[u8],
    session_key: &[u8; 32],
    direction: [u8; 4],
    frame: &EncryptedFrame,
) -> Result<Vec<u8>> {
    let cipher =
        Aes256Gcm::new_from_slice(session_key).map_err(|_| anyhow!("Geçersiz oturum anahtarı"))?;
    let nonce_bytes = nonce(direction, frame.index);
    let aad = additional_data(header_bytes, direction, frame.kind, frame.index);
    cipher
        .decrypt(
            Nonce::from_slice(&nonce_bytes),
            Payload {
                msg: &frame.ciphertext,
                aad: &aad,
            },
        )
        .map_err(|_| anyhow!("Frame kimlik doğrulaması başarısız"))
}

pub(crate) fn write_frame(stream: &mut TcpStream, body: &[u8]) -> Result<()> {
    if body.len() > MAX_FRAME_SIZE {
        bail!("Frame sınırı aşıyor: {} bayt", body.len());
    }
    let length = u32::try_from(body.len()).context("Frame uzunluğu u32 sınırını aşıyor")?;
    stream
        .write_all(&length.to_be_bytes())
        .context("Frame uzunluğu gönderilemedi")?;
    stream.write_all(body).context("Frame gönderilemedi")?;
    Ok(())
}

pub(crate) fn read_frame(stream: &mut TcpStream) -> Result<Option<EncryptedFrame>> {
    let Some(length) = read_length_prefix(stream)? else {
        return Ok(None);
    };
    let length = length as usize;
    if !(FRAME_METADATA_SIZE + GCM_TAG_SIZE..=MAX_FRAME_SIZE).contains(&length) {
        bail!("Geçersiz frame boyutu: {length}");
    }

    let mut body = vec![0u8; length];
    stream
        .read_exact(&mut body)
        .context("Frame eksik veya okunamadı")?;
    let kind = body[0];
    let index = u64::from_be_bytes(
        body[1..FRAME_METADATA_SIZE]
            .try_into()
            .expect("frame index slice has a fixed length"),
    );
    Ok(Some(EncryptedFrame {
        kind,
        index,
        ciphertext: body[FRAME_METADATA_SIZE..].to_vec(),
    }))
}

pub(crate) fn write_header(stream: &mut TcpStream, header_bytes: &[u8]) -> Result<()> {
    if header_bytes.is_empty() || header_bytes.len() > MAX_HEADER_SIZE {
        bail!("Geçersiz başlık boyutu: {}", header_bytes.len());
    }
    let length = u32::try_from(header_bytes.len()).context("Başlık çok büyük")?;
    stream
        .write_all(&length.to_be_bytes())
        .context("Başlık uzunluğu gönderilemedi")?;
    stream
        .write_all(header_bytes)
        .context("Başlık gönderilemedi")?;
    Ok(())
}

pub(crate) fn read_header_bytes(stream: &mut TcpStream) -> Result<Option<Vec<u8>>> {
    let Some(length) = read_length_prefix(stream)? else {
        return Ok(None);
    };
    let length = length as usize;
    if !(1..=MAX_HEADER_SIZE).contains(&length) {
        bail!("Geçersiz başlık boyutu: {length}");
    }
    let mut bytes = vec![0u8; length];
    stream
        .read_exact(&mut bytes)
        .context("Başlık eksik veya okunamadı")?;
    Ok(Some(bytes))
}

fn read_length_prefix(stream: &mut TcpStream) -> Result<Option<u32>> {
    let mut bytes = [0u8; 4];
    let first = stream.read(&mut bytes[..1]).context("Uzunluk okunamadı")?;
    if first == 0 {
        return Ok(None);
    }
    stream
        .read_exact(&mut bytes[1..])
        .context("Uzunluk alanı yarıda kesildi")?;
    Ok(Some(u32::from_be_bytes(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authenticated_frame_round_trip_and_tamper_detection() {
        let header = br#"{"protocol_version":1,"session_id":"001122"}"#;
        let key = [7u8; 32];
        let body = encrypt_frame(header, &key, CLIENT_DIRECTION, KIND_DATA, 3, b"hello")
            .expect("frame should encrypt");
        let mut frame = EncryptedFrame {
            kind: body[0],
            index: u64::from_be_bytes(body[1..9].try_into().unwrap()),
            ciphertext: body[9..].to_vec(),
        };

        let plaintext = decrypt_frame(header, &key, CLIENT_DIRECTION, &frame)
            .expect("frame should authenticate");
        assert_eq!(plaintext, b"hello");

        frame.ciphertext[0] ^= 1;
        assert!(decrypt_frame(header, &key, CLIENT_DIRECTION, &frame).is_err());
    }

    #[test]
    fn session_keys_are_separated() {
        let base_key = [9u8; 32];
        let first = derive_session_key(&base_key, &[1u8; 16]).unwrap();
        let second = derive_session_key(&base_key, &[2u8; 16]).unwrap();
        assert_ne!(first, second);
    }
}
