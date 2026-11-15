defmodule PublishedSmoke.MixProject do
  use Mix.Project

  # Installs ex_scylla from Hex (never from a path), so this checks the package
  # users get. `mise run verify:published` sets EX_SCYLLA_VERSION.
  def project do
    [
      app: :published_smoke,
      version: "0.1.0",
      elixir: "~> 1.15",
      deps: [{:ex_scylla, "== " <> System.get_env("EX_SCYLLA_VERSION", "0.11.0")}]
    ]
  end

  def application, do: [extra_applications: [:logger]]
end
