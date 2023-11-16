use reqwest::{Request, header};
use rand::Rng;
use rsa::RsaPrivateKey;
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::sha2::{Digest, Sha256};
use rsa::pkcs8::DecodePrivateKey;

pub(crate) async fn sign_to_reqest(req:&mut Request) -> &mut Request {
    let mut headers = req.headers_mut();

    req
}

// use ring::{rand, rsa, signature};


// たぶん署名はこれ 使い方が不明 いつかなんとかする
// fn sign_and_verify_rsa(private_key_path: &std::path::Path,
//                        public_key_path: &std::path::Path)
//                        -> Result<(), MyError> {
//     // Create an RSA keypair from the DER-encoded bytes. This example uses
//     // a 2048-bit key, but larger keys are also supported.
//     let private_key_der = read_file(private_key_path)?;
//     let key_pair = rsa::KeyPair::from_der(&private_key_der)
//         .map_err(|_| MyError::BadPrivateKey)?;

//     // Sign the message "hello, world", using PKCS#1 v1.5 padding and the
//     // SHA256 digest algorithm.
//     const MESSAGE: &'static [u8] = b"hello, world";
//     let rng = rand::SystemRandom::new();
//     let mut signature = vec![0; key_pair.public().modulus_len()];
//     key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)
//         .map_err(|_| MyError::OOM)?;

//     // Verify the signature.
//     let public_key =
//         signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256,
//                                         read_file(public_key_path)?);
//     public_key.verify(MESSAGE, &signature)
//         .map_err(|_| MyError::BadSignature)
// }

// write test
#[test]
fn some_test() {
    let headers = "test";
    let privkey = "-----BEGIN PRIVATE KEY-----
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
DeBarcSEEEOYVVW7N7pal3x6Ta2G
-----END PRIVATE KEY-----";
    let mut rng = rand::thread_rng();

    let bits = 4096;

    let private_key: RsaPrivateKey = DecodePrivateKey::from_pkcs8_pem(privkey).unwrap();
    let signing_key: SigningKey<Sha256> = SigningKey::<Sha256>::new(private_key);

    let data = headers.as_bytes();

    let signature = signing_key.sign_with_rng(&mut rng, data);
    println!("{}", signature)
}

pub(crate) fn sign_string_with_privkey(headers: &str, privkey: &str) -> String {
    // let rng = rand::SystemRandom::new();
    // let key_pair = rsa::KeyPair::from_pkcs8(privkey.as_bytes()).map_err(|_| MyError::BadPrivateKey)?;
//     let PRIV_KEY = "-----BEGIN PRIVATE KEY-----
// MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCE5OWoW+4Y/hrH
// yuyH0pk62Ot37E/u0VCXMgAEYKp6R8aiszfoipvH4RdKZcUeH15f3ZdzDFFD4gqS
// MThFM6OzTZWV0jHoa+MW3Idp5/tc+8kra0pTJuKDJjTHknjP9jXYXtrcgUMwwEfU
// eFf0FtoeHIAq/NBlXb/vnchA8g+iZ2iczxILf4tHflSXgjuSf+NRqr6YxagkcmaA
// pyYLnkzePbwBUFk0J5VIpgXhGIto0osZmrwrZ5E1bt1GQunu1tYhsVw3MSiZx/DC
// +pYyIDcsxRh32X26SWNYjXFw/6aMIu+w5L4B4WxX1QYvEJJQWKaN4a0P5hdQJz93
// /GIN4B99AgMBAAECggEAAcoSNIuI/8GmIH62jhfzaQ6EJyEhrLnD78aO0lWZ3gZ1
// jHnEx4cG6htOr2QCPNxXw+CC2kagw9JOlEwrht+sioIW1kWrrIaNWstdMGYc+pIc
// /I5tLwRE7wFL8Hb5VPV/9pnvsPc1mCRdTeVRQS+wlVC1bL7AMJPhUnY0UiQ1j9kh
// y3pouIlksPIhA5Y7mkTSKs86gL3Jte0Qa9ZIeq23C2jEU0gpf6Imx9TwOw+vMuO9
// 6GJ2ha58yuVpMPplHRXuRl2o6QlxafJDSjP/EDNH/sYhYrfJu5WLZNS3bRpko2q5
// NHYCzdkSyhGcLIbcxeAqtNjN4vVPX+gpaSNpK0QVUQKBgQC5D7dtmzV46HFByXeI
// /F2x6pO/CpbNAZ1R+eDB90K0h+AmcqdTrnr37BewO178HSxAlTQU6CyVsFmH0csM
// gSqnemGlxMFGRX+dEQReqdEg8fh5gMwalWIB0ZyvZQJsffxkwvMS73fb64fMJt2i
// TTLtitj0I/GnQLTPZm6Dj5CqrQKBgQC31fK3VfoUZoBIrdDpB8h4WH5bXEvI0jK1
// vJqjJEgkIq79SE/HbMccxip4K7uo/1GjFe4KwN7Uuhc35UwB19dBi7J21MeDJ/cG
// 5dVCntynPuWe/CCSvElGrfPmGOc6pGummxpHlIPT7fyma/yyPvnwtZ6LyAt0FUl8
// 9stz+IsyEQKBgQCz/5VqBoz8DRob4tlhZBW992u9ZY9H10otcd4qy1UWQxLCUsJt
// okf43Kotv5GjprxkFLuTmj/DWMO8V1In13qla2OO2NEkiHSXUPXvHT1Lzg7gH2Zm
// dOXe3wKHlrfEzsWvO+8xe7oZLf3nk+X+9xeR9HsQ++UPyOAU2oyjagBMAQKBgGdx
// /shnmveLzjgXhuz6MjZ2JXQndYWlsl9Np/6RVf7vfWSNIkdn0ItCf3drtIeeVEPe
// /ToT2c/+fz42yxRmbnw1rdDsXvBQttKs1dpNJoD+BZv26CVpyhn5nLsn3EXFa8Yu
// lRUeXygMTRUgwutSQLbQnEyv3rINoHKvNUEm5LzhAoGAdlHajGFACUltKKag2Cbt
// 7xFAr9m2xQXcWLxeooN6wpFlLtqmaBqbRpGu2yIOPTCxiJa2wEb3yOs4mp4smKr7
// YkFVUMDcCIh91ZQr+DeztzO/6YqxrZNQXZa/seDen3g19PQVz/uaQdNGf7P05kmL
// Vqv51od8+CZ3PJBxN/av7SU=
// -----END PRIVATE KEY-----";
//     let mut signature = vec![0; key_pair.public().modulus_len()];
//     key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)
//         .map_err(|_| MyError::OOM)?;
//     headers.insert("X-Signature", signature);
//     Ok(())

    // let mut rng = rand::thread_rng();

    // let bits = 4096;

    // let private_key: RsaPrivateKey = DecodePrivateKey::from_pkcs8_pem(privkey).unwrap();
    // let signing_key: SigningKey<Sha256> = SigningKey::<Sha256>::new(private_key); 

    // let data = headers.as_bytes();
    
    // let mut signature = signing_key.sign_with_rng(&mut rng, data);
    // Ok(signature.to_string())

    let mut rng = rand::thread_rng();

    let private_key: RsaPrivateKey = DecodePrivateKey::from_pkcs8_pem(privkey).unwrap();
    let signing_key: SigningKey<Sha256> = SigningKey::<Sha256>::new(private_key);

    let data = headers.as_bytes();

    let signature = signing_key.sign_with_rng(&mut rng, data);
    // println!("{}", signature)
    signature.to_string()

    // Err(MyError::BadPrivateKey)
}

// fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
//     use std::io::Read;

//     let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
//     let mut contents: Vec<u8> = Vec::new();
//     file.read_to_end(&mut contents).map_err(|e| MyError::IO(e))?;
//     Ok(contents)
// }