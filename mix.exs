defmodule ExScylla.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_scylla,
      version: "0.1.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      test_coverage: [tool: LcovEx, output: "cover"],
      deps: deps(),
      description: description(),
      package: package()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp description() do
    "A thin wrapper around the rust scylla crate: https://crates.io/crates/scylla."
  end

  defp package() do
    [
      # These are the default files included in the package
      files: ~w(lib priv/native/.place_holder .formatter.exs mix.exs README*
                native/ex_scylla/src native/ex_scylla/Cargo.*),
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => "https://github.com/cleaton/ex_scylla"}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.26"},
      {:lcov_ex, "~> 0.2", only: [:test], runtime: false},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
      {:benchee, "~> 1.0", only: [:bench]},
      {:erlcass, "~> 4.0", only: [:bench]}
    ]
  end
end
