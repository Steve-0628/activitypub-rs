use reqwest::Request;


pub(crate) async fn sign_to_reqest(req:&mut Request) -> &mut Request {
    let mut headers = req.headers_mut();

    req
}

use ring::{rand, rsa, signature};

// たぶん署名はこれ 使い方が不明 いつかなんとかする
fn sign_and_verify_rsa(private_key_path: &std::path::Path,
                       public_key_path: &std::path::Path)
                       -> Result<(), MyError> {
    // Create an RSA keypair from the DER-encoded bytes. This example uses
    // a 2048-bit key, but larger keys are also supported.
    let private_key_der = read_file(private_key_path)?;
    let key_pair = rsa::KeyPair::from_der(&private_key_der)
        .map_err(|_| MyError::BadPrivateKey)?;

    // Sign the message "hello, world", using PKCS#1 v1.5 padding and the
    // SHA256 digest algorithm.
    const MESSAGE: &'static [u8] = b"hello, world";
    let rng = rand::SystemRandom::new();
    let mut signature = vec![0; key_pair.public().modulus_len()];
    key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)
        .map_err(|_| MyError::OOM)?;

    // Verify the signature.
    let public_key =
        signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256,
                                        read_file(public_key_path)?);
    public_key.verify(MESSAGE, &signature)
        .map_err(|_| MyError::BadSignature)
}

#[derive(Debug)]
enum MyError {
   IO(std::io::Error),
   BadPrivateKey,
   OOM,
   BadSignature,
}

fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| MyError::IO(e))?;
    Ok(contents)
}