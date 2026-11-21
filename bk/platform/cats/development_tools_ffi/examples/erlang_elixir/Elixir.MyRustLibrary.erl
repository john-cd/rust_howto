%% TODO Reviewed ??-??-2026.

%% A minimal BEAM loader for the Rustler NIF built from this package.
-module('Elixir.MyRustLibrary').
-on_load(load_nif/0).
-export([load_nif/0, greet/1, add/2]).

load_nif() ->
    case os:getenv("RUSTLER_NIF_PATH") of
        false -> {error, "RUSTLER_NIF_PATH must name the NIF without its extension"};
        Path -> erlang:load_nif(Path, 0)
    end.

greet(_Name) -> erlang:nif_error(nif_not_loaded).
add(_Left, _Right) -> erlang:nif_error(nif_not_loaded).
