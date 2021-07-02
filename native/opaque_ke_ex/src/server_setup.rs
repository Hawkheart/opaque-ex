use crate::ciphersuite::Default;
use opaque_ke::ServerSetup;
use rand::rngs::OsRng;

#[rustler::nif]
pub fn new_server_setup() -> Vec<u8> {
    let mut rng = OsRng;
    let setup = ServerSetup::<Default>::new(&mut rng);

    setup.serialize()
}