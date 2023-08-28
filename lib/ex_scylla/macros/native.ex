defmodule ExScylla.Macros.Native do
  @scylla_version File.read!("#{File.cwd!()}/native/ex_scylla/Cargo.lock")
                  |> String.split("[[package]]")
                  |> Enum.find_value(nil, fn l ->
                    case l |> String.trim() |> String.split("\n") do
                      ["name = \"scylla\"", "version = " <> version | _] ->
                        String.trim(version, "\"")

                      _ ->
                        false
                    end
                  end)
  @r ~r"> \s*(?<res>.*)\s*=\s*.*\.(?<func>.*)\(.*"
  @spec __using__(keyword) :: {:__block__, [], [{any, any, any}, ...]}
  defmacro __using__(opts) do
    prefix = Keyword.get(opts, :prefix)
    docs_rs_path = Keyword.get(opts, :docs_rs_path)

    docs_url =
      if docs_rs_path do
        docs_url = "https://docs.rs/scylla/#{@scylla_version}" <> docs_rs_path
        Module.register_attribute(__CALLER__.module, :docs_rs_url, persist: true)
        Module.put_attribute(__CALLER__.module, :docs_rs_url, docs_url)
        docs_url
      else
        ""
      end

    Module.register_attribute(__CALLER__.module, :prefix, persist: true)
    Module.put_attribute(__CALLER__.module, :prefix, prefix)

    for {k, v} <- Keyword.drop(opts, [:prefix, :docs_rs_path]) do
      Module.register_attribute(__CALLER__.module, k, persist: true)
      Module.put_attribute(__CALLER__.module, k, v)
    end

    module_doc = """
    Wrapper around rust module
    See rust documentation for more usage details: #{docs_url}
    """

    quote do
      import ExScylla.Macros.Native
      @type opaque :: any()
      @moduledoc unquote(module_doc)
    end
  end

  defmacro native_struct(attrs \\ []) do
    keys = Keyword.keys(attrs)

    quote do
      @enforce_keys unquote(keys)
      defstruct unquote(keys)

      @type t :: %__MODULE__{
              unquote_splicing(attrs)
            }
    end
  end

  defmacro native_f(macro_args) do
    name = Keyword.fetch!(macro_args, :func)
    args = Keyword.fetch!(macro_args, :args)
    args_spec = Keyword.fetch!(macro_args, :args_spec)
    return_spec = Keyword.fetch!(macro_args, :return_spec)
    return_spec_str = Macro.to_string(return_spec)
    # doc_text = Keyword.get(macro_args, :doc_text, "")
    doc_example = Keyword.get(macro_args, :doc_example, "")

    example_setup =
      if Keyword.get(macro_args, :example_setup) do
        Module.get_attribute(__CALLER__.module, Keyword.get(macro_args, :example_setup))
        |> String.trim_trailing("\n")
      end

    doc_example =
      if doc_example != "" do
        example_wrap(doc_example, example_setup)
      else
        ""
      end

    prefix = Module.get_attribute(__CALLER__.module, :prefix)
    docs_rs_url = Module.get_attribute(__CALLER__.module, :docs_rs_url)

    doc = """
    #{if docs_rs_url, do: "See: #{docs_rs_url}#method.#{name}"}
    ```#{name}```, returns: ```#{return_spec_str}```.\n
    #{if doc_example != "", do: doc_example}
    """

    quote do
      func_name = unquote(name)
      @doc unquote(doc)
      @spec unquote(:"#{name}")(unquote_splicing(args_spec)) :: unquote(return_spec)
      def unquote(:"#{name}")(unquote_splicing(args)) do
        ExScylla.Native.unquote(:"#{prefix}_#{name}")(unquote_splicing(args))
      end
    end
  end

  @doc false
  defmacro native_f_async(macro_args) do
    name = Keyword.fetch!(macro_args, :func)
    args = Keyword.fetch!(macro_args, :args)
    args_spec = Keyword.fetch!(macro_args, :args_spec)
    return_spec = Keyword.fetch!(macro_args, :return_spec)
    return_spec_str = Macro.to_string(return_spec)

    type_map =
      case Keyword.get(macro_args, :type_map) do
        nil ->
          quote do
          end

        f ->
          quote do
            unquote(f)
          end
      end

    doc_example = Keyword.get(macro_args, :doc_example, "")

    example_setup =
      if Keyword.get(macro_args, :example_setup) do
        Module.get_attribute(__CALLER__.module, Keyword.get(macro_args, :example_setup))
        |> String.trim_trailing("\n")
      end

    prefix = Module.get_attribute(__CALLER__.module, :prefix)
    docs_rs_url = Module.get_attribute(__CALLER__.module, :docs_rs_url)

    async_doc = """
    #{if docs_rs_url, do: "See: #{docs_rs_url}#method.#{name}"}

    Async version of `#{name}`, returns: `{:ok, opaque} | {:error, any()}`\n
    Actual `result` (`#{return_spec_str}`) is sent to the calling process:\n
    #{if doc_example != "", do: sync_to_async_example(to_string(name), example_wrap(doc_example, example_setup))}
    ```

    """

    sync_doc = """
    #{if docs_rs_url, do: "See: #{docs_rs_url}#method.#{name}"}

    Sync version of #{name}\n
    Returns result (`#{return_spec_str}`)\n
      or `{:error, :timeout}` after `timeout_ms`.

    #{if doc_example != "", do: example_wrap(doc_example, example_setup)}
    """

    quote do
      func_name = unquote(name)
      @doc unquote(async_doc)
      @spec unquote(:"async_#{name}")(unquote_splicing(args_spec), opaque()) ::
              {:ok, opaque()} | {:error, any()}
      def unquote(:"async_#{name}")(
            unquote_splicing(args),
            opaque \\ {unquote(:"#{name}"), make_ref()}
          ) do
        unquote(type_map)

        case ExScylla.Native.unquote(:"#{prefix}_#{name}")(opaque, unquote_splicing(args)) do
          :ok -> {:ok, opaque}
          err -> err
        end
      end

      @doc unquote(sync_doc)
      @spec unquote(:"#{name}")(unquote_splicing(args_spec), pos_integer()) ::
              unquote(return_spec) | {:error, :timeout}
      def unquote(:"#{name}")(unquote_splicing(args), timeout_ms \\ 5_000) do
        case unquote(:"async_#{name}")(unquote_splicing(args)) do
          {:ok, opaque} ->
            receive do
              {^opaque, result} -> result
            after
              timeout_ms -> {:error, :timeout}
            end

          err ->
            err
        end
      end
    end
  end

  defp example_wrap("", _), do: ""

  defp example_wrap(example, nil) do
    """
    ## Example
    ```
    #{example}
    ```
    """
  end

  defp example_wrap(example, setup) do
    """
    ## Example
    ```
    #{setup}
    #{example}
    ```
    """
  end

  defp sync_to_async_example(func, example) do
    String.split(example, "\n")
    |> Enum.map(fn str ->
      m = Regex.named_captures(@r, str)

      if m != nil and Map.get(m, "func") == func do
        async_f = "async_" <> func
        result_var = Map.get(m, "res")

        str =
          String.replace(str, result_var, "{:ok, opaque} ", global: false)
          |> String.replace(func, async_f, global: false)

        """
        #{str}
        iex> #{result_var}= receive do
        ...>   {^opaque, r} -> r
        ...>  after
        ...>    5_000 -> :timeout
        ...>  end
        """
        |> String.trim_trailing("\n")
      else
        str
      end
    end)
    |> Enum.join("\n")
  end
end
