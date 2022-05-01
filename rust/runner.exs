Mix.install([
  {:wasmex, "~> 0.7.0"},
  {:jason, "~> 1.3"}
])

defmodule Runner do
  @memory_index 0
  @function_name "run"

  def run(file, args) do
    {:ok, bytes} = File.read(file)
    {:ok, module} = Wasmex.Module.compile(bytes)

    {:ok, instance} = Wasmex.start_link(%{module: module, imports: imports()})
    {:ok, memory} = Wasmex.memory(instance, :uint8, @memory_index)

    try do
      function_args = prepare_args(memory, args)

      {:ok, _} = Wasmex.call_function(instance, @function_name, function_args)

      receive do
        {:outputs, outputs} -> {:ok, outputs}
      after
        5000 -> {:error, :timeout}
      end
    after
      GenServer.stop(instance)
    end
  end

  defp prepare_args(memory, args) do
    binary = Jason.encode!(args)
    Wasmex.Memory.write_binary(memory, @memory_index, binary)

    [@memory_index, div(bit_size(binary), 8)]
  end

  defp imports() do
    %{env: %{hostcall_set_outputs: set_outputs(self())}}
  end

  defp set_outputs(pid) do
    {
      :fn,
      [:i32, :i32],
      [],
      fn context, ptr, len ->
        binary = Wasmex.Memory.read_binary(context.memory, ptr, len)
        outputs = Jason.decode!(binary)

        send(pid, {:outputs, outputs})

        nil
      end
    }
  end
end

IO.puts("""
To test the wasm, run the following command:

```elixir
wasm_file = "/path/to/program.wasm"
value_presenters = [%{type: "literal", field_type: "single_line_field", value: "Hello"}]

Runner.run(wasm_file, value_presenters)
```
""")
