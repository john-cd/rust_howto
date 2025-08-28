defmodule MyRustLibrary do
  use Rustler.NIF

  @nif greet/1 do
    {:ok, name} -> MyRustLibrary.greet(name)
  end

  @nif add/2 do
    {:ok, a, b} -> MyRustLibrary.add(a, b)
  end

  def nif_error(reason) do
    {:error, reason}
  end

  defp greet(name) do
    NIF.load("my_rust_library", 0)
    |> NIF.invoke(:greet, [name])
  end

  defp add(a, b) do
    NIF.load("my_rust_library", 0)
    |> NIF.invoke(:add, [a, b])
  end
end
