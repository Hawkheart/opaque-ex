# OPAQUE (Elixir Bindings)

Simple Elixir/Erlang bindings to the [opaque-ke](https://github.com/novifinancial/opaque-ke) Rust implementation of the OPAQUE protocol. These bindings loosely based (and intended to be compatible with) [the WebAssembly/JavaScript bindings.](https://github.com/marucjmar/opaque-wasm)

**These bindings are not suitable for serious use; they have not been thoroughly tested, and the interface is likely to change.**

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `opaque` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:opaque, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/opaque](https://hexdocs.pm/opaque).

