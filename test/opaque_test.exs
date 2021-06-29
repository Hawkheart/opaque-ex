defmodule OpaqueTest do
  use ExUnit.Case
  doctest Opaque

  test "works end to end" do
    server_keys = Opaque.generate_random_keypair()

    # Full registration flow works properly.
    assert {:ok, client_register_start_result} = Opaque.client_register_start("password!!")
    assert {:ok, server_register_start_result} = Opaque.server_register_start(server_keys, client_register_start_result.message)
    assert {:ok, client_register_finish_result} = Opaque.client_register_finish(client_register_start_result.state, server_register_start_result.message)
    assert {:ok, registration} = Opaque.server_register_finish(server_register_start_result.state, client_register_finish_result.message)


    #
    assert {:ok, client_login_start_result} = Opaque.client_login_start("password!!")
    assert {:ok, server_login_start_result} = Opaque.server_login_start(registration, server_keys, client_login_start_result.message)
    assert {:ok, client_login_finish_result} = Opaque.client_login_finish(client_login_start_result.state, server_login_start_result.message)
    assert {:ok, server_login_finish_result} = Opaque.server_login_finish(server_login_start_result.state, client_login_finish_result.message)

    # Output: The client's export key remains the same between signup and login.
    # The server and the client have a shared key.
    assert client_register_finish_result.export_key == client_login_finish_result.export_key
    assert server_login_finish_result == client_login_finish_result.session_key


    # An incorrect password submission does NOT work.
    assert {:ok, client_login_start_result} = Opaque.client_login_start("incorrect")
    assert {:ok, server_login_start_result} = Opaque.server_login_start(registration, server_keys, client_login_start_result.message)
    assert {:error, :invalid_login} = Opaque.client_login_finish(client_login_start_result.state, server_login_start_result.message)
  end
end
