use crate::ciphersuite::Default;
use crate::errors::OpaqueError;
use opaque_ke::{ServerRegistration, RegistrationRequest, RegistrationUpload, ServerSetup};

#[derive(rustler::NifStruct)]
#[module = "Opaque.ServerRegistrationStartResult"]
pub struct ServerRegistrationStartResult {
    state: Vec<u8>,
    message: Vec<u8>
}

#[rustler::nif]
pub fn server_register_start(setup: Vec<u8>, client_message: Vec<u8>, identifier: Vec<u8>) -> Result<Vec<u8>, OpaqueError> {
    let setup = ServerSetup::deserialize(&setup)?;

    let req = RegistrationRequest::deserialize(&client_message)?;

    let result = ServerRegistration::<Default>::start(
        &setup,
        req,
        &identifier
    )?;

    Ok(result.message.serialize())
}

#[rustler::nif]
pub fn server_register_finish(upload: Vec<u8>) -> Result<Vec<u8>,OpaqueError> {
    let message = RegistrationUpload::deserialize(&upload)?;

    let registration = ServerRegistration::<Default>::finish(message);

    Ok(registration.serialize())
}