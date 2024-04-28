defmodule ExStemmers.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_stemmers,
      version: "0.1.0",
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package(),
      description: description()
    ]
  end

  defp description do
    "ex_stemmers wraps the rust-stemmers crate"
  end

  defp package do
    [
      # These are the default files included in the package
      files: [
        "lib",
        "mix.exs",
        # "native/exstemmers_native/.cargo",
        "native/exstemmers_native/src",
        "native/exstemmers_native/Cargo*",
        "README*"
      ],
      licenses: ["BSD-3", "MIT"],
      links: %{
        "GitHub" => "https://github.com/dkuku/ex_stemmers"
      }
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.32.1", runtime: false},
      {:ex_doc, "~> 0.31", only: :dev, runtime: false},
      {:styler, "~> 0.11", only: [:dev, :test], runtime: false},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false}
    ]
  end
end
