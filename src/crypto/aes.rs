use openssl::symm::{Cipher, Crypter, Mode};
use std::error::Error as StdErr;
use std::fs::File;
use std::io::{Read, Write};

pub struct AesCipher<'a> {
    key: &'a [u8],
    iv: &'a [u8],
}

pub enum AesMode {
    CTR,
    CFB,
    CBC,
}

impl<'a> AesCipher<'a> {
    pub fn new(key: &'a [u8], iv: &'a [u8]) -> AesCipher<'a> {
        AesCipher { key, iv }
    }

    pub fn encrypt(
        &mut self,
        mode: AesMode,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<usize, Box<dyn StdErr>> {
        self.process_data(mode, input, output, true)
    }

    pub fn decrypt(
        &mut self,
        mode: AesMode,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<usize, Box<dyn StdErr>> {
        self.process_data(mode, input, output, false)
    }

    fn process_data(
        &mut self,
        mode: AesMode,
        input: &[u8],
        output: &mut [u8],
        encrypt: bool,
    ) -> Result<usize, Box<dyn StdErr>> {
        let mut context = AesCipher::get_cipher_context(self.key, self.iv, mode, encrypt)?;
        let n = context.update(input, output)?;
        context.finalize(&mut output[n..]).map_err(|e| e.into())
    }

    pub fn encrypt_stream(
        &mut self,
        mode: AesMode,
        input: impl Read,
        output: impl Write,
    ) -> Result<(), Box<dyn StdErr>> {
        self.process_stream(mode, input, output, true)
    }

    pub fn decrypt_stream(
        &mut self,
        mode: AesMode,
        input: impl Read,
        output: impl Write,
    ) -> Result<(), Box<dyn StdErr>> {
        self.process_stream(mode, input, output, false)
    }

    fn process_stream(
        &mut self,
        mode: AesMode,
        mut input: impl Read,
        mut output: impl Write,
        encrypt: bool,
    ) -> Result<(), Box<dyn StdErr>> {
        let mut context = AesCipher::get_cipher_context(self.key, self.iv, mode, encrypt)?;
        let mut buff_in: [u8; 4096] = [0; 4096];
        let mut buff_out: [u8; 4096] = [0; 4096];
        let mut n_read = 0;
        let mut n_write = 0;
        loop {
            n_read = input.read(&mut buff_in)?;
            if n_read == 0 {
                break;
            }
            n_write = context.update(&buff_in[..n_read], &mut buff_out)?;
            output.write(&buff_out[..n_write])?;
        }
        let n = context.finalize(&mut buff_out).map_err(|e| Box::new(e))?;
        output.write(&mut buff_out[..n]).map_err(|e| Box::new(e))?;
        output.flush().map_err(|e| "".into())
    }

    pub fn encrypt_file(
        &mut self,
        mode: AesMode,
        input: &File,
        output: &File,
    ) -> Result<(), Box<dyn StdErr>> {
        self.encrypt_stream(mode, input, output)?;
        output.sync_all().map_err(|e| e.into())
    }

    pub fn decrypt_file(
        &mut self,
        mode: AesMode,
        input: &File,
        output: &File,
    ) -> Result<(), Box<dyn StdErr>> {
        self.decrypt_stream(mode, input, output)?;
        output.sync_all().map_err(|e| e.into())
    }

    /********
       AesMode::CTR --> java: AES/CTR/NoPadding,
       AesMode::CFB --> java: AES/CFB/NoPadding,
       AesMode::CBC --> java: AES/CBC/PKCS5Padding,
    ******/

    fn get_cipher_context(
        key: &[u8],
        iv: &[u8],
        mode: AesMode,
        encrypt: bool,
    ) -> Result<Crypter, Box<dyn StdErr>> {
        let cipher = match mode {
            AesMode::CTR => Cipher::aes_256_ctr(),
            AesMode::CBC => Cipher::aes_256_cbc(),
            AesMode::CFB => Cipher::aes_256_cfb128(),
        };
        let mode = if encrypt {
            Mode::Encrypt
        } else {
            Mode::Decrypt
        };
        Crypter::new(cipher, mode, key, Some(iv))
            .map(|mut c| {
                if matches!(AesMode::CBC, mode) {
                    c.pad(true);
                }
                c
            })
            .map_err(|e| e.into())
    }
}
