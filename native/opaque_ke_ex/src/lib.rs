pub mod client_login;
pub mod client_registration;
pub mod ciphersuite;
pub mod server_setup;
pub mod server_registration;
pub mod server_login;
pub mod errors;

rustler::init!(
    "Elixir.Opaque",
    [
        client_registration::client_register_start,
        client_registration::client_register_finish,
        client_login::client_login_start,
        client_login::client_login_finish,
        server_setup::new_server_setup,
        server_registration::server_register_start,
        server_registration::server_register_finish,
        server_login::server_login_start,
        server_login::server_login_finish
    ]
);
