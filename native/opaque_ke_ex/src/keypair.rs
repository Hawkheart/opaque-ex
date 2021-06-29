use opaque_ke::ciphersuite::CipherSuite;

use crate::ciphersuite::Default;
use rand::rngs::OsRng;

pub type NativeKeyPair = opaque_ke::keypair::KeyPair<curve25519_dalek::ristretto::RistrettoPoint>;

#[derive(rustler::NifStruct)]
#[module="Opaque.KeyPair"]
pub struct KeyPair {
    pub public: Vec<u8>,
    pub private: Vec<u8>
}

impl From<NativeKeyPair> for KeyPair {
    fn from(key: NativeKeyPair) -> KeyPair {

        KeyPair {
            private: key.private().as_slice().into(),
            public: key.public().as_slice().into()
        }
    }
}

#[rustler::nif]
pub fn generate_random_keypair() -> KeyPair {
    let mut rng = OsRng;

    let kp = Default::generate_random_keypair(&mut rng);

    return kp.into();
}

#[rustler::nif]
pub fn import_private_key(private_key: Vec<u8>) -> KeyPair {
    let kp = NativeKeyPair::from_private_key_slice(&private_key).unwrap();

    return kp.into();
}