defmodule ExStemmersTest do
  use ExUnit.Case

  test "returns the stemmed word" do
    assert ExStemmers.Native.stemmer("interpolate") == "interpol"
    assert ExStemmers.Native.stemmer("interpolate", language: :norwegian) == "interpolat"
  end
end
