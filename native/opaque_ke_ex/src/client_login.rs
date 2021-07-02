use crate::ciphersuite::Default;
use crate::errors::OpaqueError;
use opaque_ke::{ClientLogin, CredentialResponse, ClientLoginFinishParameters};
use rand::rngs::OsRng;

#[derive(rustler::NifStruct)]
#[module = "Opaque.ClientLoginStartResult"]
pub struct ClientLoginStartResult {
    state: Vec<u8>,
    message: Vec<u8>
}

#[derive(rustler::NifStruct)]
#[module = "Opaque.ClientLoginFinishResult"]
pub struct ClientLoginFinish {
    message: Vec<u8>,
    server_s_pk: Vec<u8>,
    session_key: Vec<u8>,
    export_key: Vec<u8>
}

#[rustler::nif]
pub fn client_login_start(password: &str) -> Result<ClientLoginStartResult, OpaqueError> {
    let mut rng = OsRng;
    let res = ClientLogin::<Default>::start(&mut rng, password.as_bytes())?;

    Ok(ClientLoginStartResult {
        state: res.state.serialize(),
        message: res.message.serialize()
    })
}

#[rustler::nif(schedule="DirtyCpu")]
pub fn client_login_finish(state: Vec<u8>, server_message: Vec<u8>) -> Result<ClientLoginFinish, OpaqueError> {

    let cl = ClientLogin::<Default>::deserialize(&state)?;

    let cr = CredentialResponse::deserialize(&server_message)?;

    let res = cl.finish(cr, ClientLoginFinishParameters::Default)?;

    Ok(ClientLoginFinish {
        message: res.message.serialize(),
        server_s_pk: res.server_s_pk.to_vec(),
        session_key: res.session_key.to_vec(),
        export_key: res.export_key.to_vec()
    })
}