use rsa::RsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::signature::RandomizedSigner;
use rsa::sha2::Sha256;
use rsa::pkcs8::DecodePrivateKey;

// write test
#[test]
fn some_test() {
    let headers = "test";
    // this key is safe to leak
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

    let mut rng = rand::thread_rng();

    let private_key: RsaPrivateKey = DecodePrivateKey::from_pkcs8_pem(privkey).unwrap();
    let signing_key: SigningKey<Sha256> = SigningKey::<Sha256>::new(private_key);

    let data = headers.as_bytes();

    let signature = signing_key.sign_with_rng(&mut rng, data);
    // println!("{}", signature)
    signature.to_string()

    // Err(MyError::BadPrivateKey)
}
