use opaque_ke::errors::{PakeError, ProtocolError, InternalPakeError};

#[derive(rustler::NifUnitEnum)]
pub enum OpaqueError {
    InvalidLogin,
    Unknown
}

impl From<ProtocolError> for OpaqueError {
    fn from(err: ProtocolError) -> Self {
        match err {
            ProtocolError::VerificationError(pake_err) => {
                match pake_err {
                    PakeError::InvalidLoginError => Self::InvalidLogin,
                    _ => Self::Unknown
                }
            }
            _ => Self::Unknown
        }
    }
}

impl From<InternalPakeError> for OpaqueError {
    fn from(_err: InternalPakeError) -> Self {
        Self::Unknown
    }
}