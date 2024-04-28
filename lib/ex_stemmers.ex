defmodule ExStemmers.Native do
  use Rustler,
    otp_app: :ex_stemmers,
    crate: "exstemmers_native"

  def stemmer(text), do: stemmer(text, language: :english)
  def stemmer(_, _), do: err()
  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
