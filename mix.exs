defmodule ExScylla.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_scylla,
      version: "0.5.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      test_coverage: [tool: LcovEx, output: "cover"],
      deps: deps(),
      description: description(),
      package: package(),
      aliases: aliases(),
      elixirc_paths: elixirc_paths(Mix.env())
    ]
  end

  defp elixirc_paths(env) when env in [:test, :bench], do: ["lib", "test/utils"]
  defp elixirc_paths(_), do: ["lib"]

  defp aliases do
    [
      test: &run_test/1,
      "test.wrapper": &run_test_wrapper/1,
      bench: &run_bench/1
    ]
  end

  defp run_test(args) do
    args = if IO.ANSI.enabled?(), do: ["--color" | args], else: ["--no-color" | args]

    {_, res} =
      System.cmd("mix", ["test.wrapper" | args],
        into: IO.binstream(:stdio, :line),
        env: [
          {"MIX_ENV", to_string(Mix.env())},
          {"RUSTFLAGS", "-C instrument-coverage"}
        ]
      )

    if res > 0 do
      System.at_exit(fn _ -> exit({:shutdown, 1}) end)
    end
  end

  defp run_test_wrapper(args) do
    Mix.Tasks.Test.run(args)
  end

  defp run_bench(args) do
    case args do
      [bench_name] ->
        filename = "bench/#{bench_name}.exs"
        if File.exists?(filename) do
          Mix.shell().info("Running benchmark: #{filename}")
          {_, res} = System.cmd("mix", ["run", filename],
            into: IO.binstream(:stdio, :line),
            env: [{"MIX_ENV", "bench"}]
          )
          if res > 0 do
            System.at_exit(fn _ -> exit({:shutdown, 1}) end)
          end
        else
          Mix.shell().error("Benchmark file not found: #{filename}")
        end
      _ ->
        Mix.shell().error("Usage: mix bench <benchmark_name>")
    end
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
      {:rustler, "~> 0.29.1"},
      {:testcontainers, "~> 1.11", only: [:test, :bench]},
      {:lcov_ex, "~> 0.3", only: [:test], runtime: false},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
      {:benchee, "~> 1.0", only: [:bench]},
      {:erlcass, "~> 4.1", only: [:bench], runtime: false}
    ]
  end
end
