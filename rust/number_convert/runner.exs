Mix.install([
  {:wasmex, "~> 0.7.1"}
])


defmodule Runner do

  def run do
    use Bitwise
    {:ok, bytes } = File.read("number_convert.wasm")
    {:ok, module} = Wasmex.Module.compile(bytes)
    {:ok, instance } = Wasmex.start_link(%{module: module}) # starts a GenServer running this WASM instance
    {:ok, memory} = Wasmex.memory(instance, :uint8, 0)
    {:ok, [pointer_len]} = Wasmex.call_function(instance, "int_number_to_chinese", [2617677])
    len = pointer_len &&& 0xff;
    pointer = pointer_len >>> 8;
    returned_string = Wasmex.Memory.read_string(memory, pointer, len)
  end
end
