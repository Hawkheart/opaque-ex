defmodule Opaque do
  use Rustler, otp_app: :opaque, crate: "opaque_ke_ex"

  @moduledoc """
  Documentation for `Opaque`.
  """

  def client_register_start(_password), do: :erlang.nif_error(:nif_not_loaded)
  def client_register_finish(_state, _server_message), do: :erlang.nif_error(:nif_not_loaded)

  def server_register_start(_keypair, _client_message), do: :erlang.nif_error(:nif_not_loaded)
  def server_register_finish(_state, _client_register_message), do: :erlang.nif_error(:nif_not_loaded)

  def client_login_start(_password), do: :erlang.nif_error(:nif_not_loaded)
  def client_login_finish(_state, _server_message), do: :erlang.nif_error(:nif_not_loaded)

  def server_login_start(_registration, _keypair, _message), do: :erlang.nif_error(:nif_not_loaded)
  def server_login_finish(_state, _client_message), do: :erlang.nif_error(:nif_not_loaded)

  def generate_random_keypair(), do: :erlang.nif_error(:nif_not_loaded)

  def import_private_key(_key), do: :erlang.nif_error(:nif_not_loaded)
end
