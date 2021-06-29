use crate::keypair;
use crate::ciphersuite::Default;
use crate::errors::OpaqueError;
use rand::rngs::OsRng;

use opaque_ke::{ServerRegistration, ServerLogin, ServerLoginStartParameters, CredentialRequest, CredentialFinalization};

#[derive(rustler::NifStruct)]
#[module = "Opaque.ServerLoginStartResult"]
pub struct ServerLoginStartResult {
    state: Vec<u8>,
    message: Vec<u8>,
    plain_info: Vec<u8>
}

#[rustler::nif]
pub fn server_login_start(registration: Vec<u8>, keypair: keypair::KeyPair, message: Vec<u8>) -> Result<ServerLoginStartResult, OpaqueError> {
    let mut rng = OsRng;

    let native_keys = keypair::NativeKeyPair::from_private_key_slice(&keypair.private)?;
    
    let password_file = ServerRegistration::<Default>::deserialize(&registration)?;

    let req = CredentialRequest::deserialize(&message)?;

    let res = ServerLogin::start(&mut rng, password_file, native_keys.private(), req, ServerLoginStartParameters::default())?;

    Ok(ServerLoginStartResult {
        state: res.state.serialize(),
        message: res.message.serialize(),
        plain_info: res.plain_info.to_vec()
    })
}

#[rustler::nif]
pub fn server_login_finish(state: Vec<u8>, message: Vec<u8>) -> Result<Vec<u8>, OpaqueError> {
    let server_login = ServerLogin::<Default>::deserialize(&state)?;

    let req = CredentialFinalization::deserialize(&message)?;

    let res = server_login.finish(req)?;

    Ok(res.session_key.to_vec())
}