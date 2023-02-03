[GET /api/v4/yaw/flows/:id/journeys/:journey_id](https://byzanteam.github.io/skylark-v4-api-doc/#vertices)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|flow id|Integer|流程ID|
|vertices id|Integer|节点ID|
|authorization token|String|授权令牌|

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
    "value" => ["Authorization", "b01110629541b3eb51697db5a05dd2388aed11a58c81a75e9c08347bc30a09e6:eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lc3BhY2VfaWQiOjF9.wj9V0ZVOOzSPuRYztizJL_5w0u8aJKb05Z73tEV_HuY"]
  },
  %{
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 1799
  },
  %{
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 7177
  }
]
```

## Outputs
```elixir
[
  %{
    "field_type" => "SINGLE_LINE_FIELD",
    "type" => "LITERAL",
    "value" => "Jason.decode!({\"id\":7177,\"name\":\"开始节点\",\"type\":\"YetAnotherWorkflow::Vertex::Initial\",\"alias_name\":\"\",\"push_settings\":[{\"waiting\":0,\"push_time\":\"created\",\"push_type\":\"Push::CustomMessage\",\"push_scope\":\"review\"},{\"waiting\":0,\"push_time\":\"updated\",\"push_type\":\"Push::CustomMessage\",\"push_scope\":\"cc\"},{\"waiting\":0,\"push_time\":\"time_up\",\"push_type\":\"Push::CustomMessage\",\"push_scope\":\"review\"},{\"waiting\":0,\"push_time\":\"time_up\",\"push_type\":\"Push::CustomMessage\",\"push_scope\":\"cc\"}],\"fields\":[{\"id\":7957,\"title\":\"输入一个测试文本\",\"description\":null,\"type\":\"Field::TextField\",\"position\":0,\"validations\":[],\"other_option\":null,\"visibility\":\"public_visibility\",\"marked\":false,\"settings\":{\"layout\":\"block\",\"char_size_limit_settings\":{},\"other_option_settings\":{\"limit\":{}},\"length_limit\":{},\"limit_settings\":{},\"enable_wechat_scan\":false},\"detail_id\":null,\"identity_key\":\"f3fbe5b4bdbd4019a622a5bc3e4ee915\",\"data\":{}}],\"user_boundary\":{\"id\":9936,\"organization_ids\":[],\"user_tag_ids\":[],\"user_tagged_type\":\"intersection\",\"only_binded\":false,\"overlord_type\":\"YetAnotherWorkflow::Vertex\",\"overlord_id\":7177,\"created_at\":\"2023-01-05T11:03:12.465+08:00\",\"updated_at\":\"2023-01-05T11:14:17.276+08:00\",\"namespace_id\":1,\"user_ids\":[],\"status\":\"default\",\"append_count\":0,\"cached_organization_ids\":[],\"cached_user_ids\":[],\"extra_boundary\":{\"reviewer_vertex_ids\":[]},\"type\":\"UserBoundary::VertexReview\"}})"
  }
]
```
