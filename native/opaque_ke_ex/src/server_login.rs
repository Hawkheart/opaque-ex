use crate::ciphersuite::Default;
use crate::errors::OpaqueError;
use rand::rngs::OsRng;

use opaque_ke::{ServerRegistration, ServerLogin, ServerLoginStartParameters, CredentialRequest, CredentialFinalization, ServerSetup};

#[derive(rustler::NifStruct)]
#[module = "Opaque.ServerLoginStartResult"]
pub struct ServerLoginStartResult {
    state: Vec<u8>,
    message: Vec<u8>
}

#[rustler::nif]
pub fn server_login_start(server_setup: Vec<u8>, registration: Vec<u8>, identifier: Vec<u8>, message: Vec<u8>) -> Result<ServerLoginStartResult, OpaqueError> {
    let mut rng = OsRng;

    let server_setup = ServerSetup::deserialize(&server_setup)?;

    let password_file = ServerRegistration::<Default>::deserialize(&registration)?;

    let req = CredentialRequest::deserialize(&message)?;

    let res = ServerLogin::start(&mut rng, &server_setup, Some(password_file), req, &identifier, ServerLoginStartParameters::default())?;

    Ok(ServerLoginStartResult {
        state: res.state.serialize(),
        message: res.message.serialize(),
    })
}

#[rustler::nif]
pub fn server_login_finish(state: Vec<u8>, message: Vec<u8>) -> Result<Vec<u8>, OpaqueError> {
    let server_login = ServerLogin::<Default>::deserialize(&state)?;

    let req = CredentialFinalization::deserialize(&message)?;

    let res = server_login.finish(req)?;

    Ok(res.session_key.to_vec())
}