# flow util

## ExtractError

```rust
#[derive(Debug)]
struct ExtractError {
    message: String,
}
impl ExtractError {
	fn new(message: String) -> Self {
	    Self {message}
	}
}
```
The first two functions are mainly to resolve the encapsulated enum to the base data type, number_to_string_id is to convert data of type Number to integer data and to string type, and the last one is to extract the public steps and simplify the code written during the network request
## extract_single_line_field_value

### Example

```rust
fn extract_single_line_field_value(value_present) -> Result<Option<&str>, ExtractError> {todo!}
```

## extract_numeric_field_value

### Example

```rust
fn extract_numeric_field_value(value_present) -> Result<Option<&Number>, ExtractError> {todo!}
```

## number_to_string_id

### Example

```rust
fn number_to_string_id(number_value: &Number) -> String {todo!}
```

## response_to_outputs

### Example

```rust
fn response_to_outputs(response: &NetworkingResponse) -> Outputs{todo!}
```
