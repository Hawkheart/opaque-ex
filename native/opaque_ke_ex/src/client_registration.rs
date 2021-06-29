use crate::ciphersuite::Default;
use crate::errors::OpaqueError;
use opaque_ke::{ClientRegistration, ClientRegistrationFinishParameters, RegistrationResponse};
use rand::rngs::OsRng;

#[derive(rustler::NifStruct)]
#[module = "Opaque.ClientRegistrationStartResult"]
pub struct ClientRegistrationStartResult {
    state: Vec<u8>,
    message: Vec<u8>
}

#[derive(rustler::NifStruct)]
#[module = "Opaque.ClientRegistrationFinishResponse"]
pub struct ClientRegistrationFinishResponse {
    message: Vec<u8>,
    export_key: Vec<u8>
}

#[rustler::nif]
pub fn client_register_start(password: &str) -> Result<ClientRegistrationStartResult, OpaqueError> {
    let mut rng = OsRng;

    let client_start_result =  ClientRegistration::<Default>::start(&mut rng, password.as_bytes())?;

    let state = client_start_result.state.serialize();
    let message = client_start_result.message.serialize();

    Ok(ClientRegistrationStartResult {state, message})
}

#[rustler::nif]
pub fn client_register_finish(state: Vec<u8>, server_message: Vec<u8>) -> Result<ClientRegistrationFinishResponse, OpaqueError> {
    let mut rng = OsRng;

    let registration = ClientRegistration::<Default>::deserialize(&state)?;

    let reg_response = RegistrationResponse::<Default>::deserialize(&server_message)?;

    let resp = registration.finish(&mut rng, reg_response, ClientRegistrationFinishParameters::Default)?;

    Ok(ClientRegistrationFinishResponse {
        message: resp.message.serialize(),
        export_key: resp.export_key.to_vec()
    })
}