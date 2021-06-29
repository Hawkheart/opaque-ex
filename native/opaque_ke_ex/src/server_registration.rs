use crate::ciphersuite::Default;
use crate::keypair::{KeyPair, NativeKeyPair};
use crate::errors::OpaqueError;
use opaque_ke::{ServerRegistration, RegistrationRequest, RegistrationUpload};
use rand::rngs::OsRng;

#[derive(rustler::NifStruct)]
#[module = "Opaque.ServerRegistrationStartResult"]
pub struct ServerRegistrationStartResult {
    state: Vec<u8>,
    message: Vec<u8>
}

#[rustler::nif]
pub fn server_register_start(keypair: KeyPair, client_message: Vec<u8>) -> Result<ServerRegistrationStartResult, OpaqueError> {
    let mut rng = OsRng;

    let native = NativeKeyPair::from_private_key_slice(&keypair.private)?;

    let req = RegistrationRequest::deserialize(&client_message)?;

    let result = ServerRegistration::<Default>::start(
        &mut rng,
        req,
        native.public()
    )?;

    let r = ServerRegistrationStartResult { state: result.state.serialize(), message: result.message.serialize() };

    Ok(r)
}

#[rustler::nif]
pub fn server_register_finish(state: Vec<u8>, upload: Vec<u8>) -> Result<Vec<u8>,OpaqueError> {
    let reg = ServerRegistration::<Default>::deserialize(&state)?;

    let u = RegistrationUpload::deserialize(&upload)?;

    let result = reg.finish(u)?;

    Ok(result.serialize())
}