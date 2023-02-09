# audit flow

[PUT /api/v4/yaw/journeys/:journey_id/assignments/:id](https://byzanteam.github.io/skylark-v4-api-doc/#2bcd4ecdd9)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|authorization token|String|授权令牌|
|content type|String|声明为json|
|journeys id|Integer|流程记录ID|
|assignment id|Integer|节点ID|
|body|String|Http request body|

## Inputs
```elixir
[
  %{
    "field_type" => "single_line_field",
    "type" => "literal",
    "value" => "https://beta.skylarkly.com"
  },
  %{
    "field_type" => "single_line_field",
    "type" => "literal",
    "value" => ["Authorization", "b01110629541b3eb51697db5a05dd2388aed11a58c81a75e9c08347bc30a09e6:eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lc3BhY2VfaWQiOjF9.wj9V0ZVOOzSPuRYztizJL_5w0u8aJKb05Z73tEV_HuY", "Content-Type", "application/json"]
  },
  %{
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 2883
  },
  %{
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 9823
  },
  %{
    "field_type" => "single_line_field",
    "type" => "literal",
    "value" => "{\"assignment\":{\"response_attributes\":{\"entries_attributes\":[]},\"comment\":\"处理意见\",\"operation\":\"approve\",\"next_vertex_id\":\"7202\"},\"user_id\":79}",
]
```

## Outputs
### 因为没有权限
```elixir
{
    "type": "user_input",
    "errors": {
        "base": [
            "操作不被允许"
        ]
    }
}
```
