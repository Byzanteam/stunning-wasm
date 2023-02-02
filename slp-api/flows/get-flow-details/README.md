# get flow details

[GET /api/v4/yaw/flows/:id](https://byzanteam.github.io/skylark-v4-api-doc/#3820c9e3a0)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|flow id|Integer|流程ID|
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
  }
]
```

## Outputs
```elixir
[
  %{
    "field_type" => "SINGLE_LINE_FIELD",
    "type" => "LITERAL",
    "value" => Jason.decode!("{\"id\":1799,\"title\":\"wasm-test\",\"html_body\":null,\"external_settings\":{},\"identity_vertify_type\":\"off\",\"logic_settings\":{\"valid\":true,\"constraints\":{},\"controlled_field_ids\":[]},\"fields\":[{\"id\":7957,\"title\":\"输入一个测试文本\",\"description\":null,\"type\":\"Field::TextField\",\"position\":0,\"validations\":[],\"other_option\":null,\"visibility\":\"public_visibility\",\"marked\":false,\"settings\":{\"layout\":\"block\",\"char_size_limit_settings\":{},\"other_option_settings\":{\"limit\":{}},\"length_limit\":{},\"limit_settings\":{},\"enable_wechat_scan\":false},\"detail_id\":null,\"identity_key\":\"f3fbe5b4bdbd4019a622a5bc3e4ee915\",\"data\":{}}],\"vertices\":[{\"id\":7177,\"name\":\"开始节点\",\"type\":\"YetAnotherWorkflow::Vertex::Initial\",\"field_ids\":[],\"operations\":{\"route\":{\"name\":\"选路\",\"enabled\":true},\"stash\":{\"name\":\"保存\",\"enabled\":true},\"cancel\":{\"name\":\"撤销\",\"enabled\":true},\"remind\":{\"name\":\"催促\",\"enabled\":true},\"comment\":{\"name\":\"处理意见\",\"enabled\":true},\"propose\":{\"name\":\"发起\",\"enabled\":true},\"esignature\":{\"name\":\"电子签字\",\"enabled\":false}},\"settings\":{\"carbon_copy\":{\"enable_manual\":false},\"refuse_mode\":1,\"assign_manually\":0,\"comment_present\":{\"refuse\":false,\"approve\":false,\"transfer\":false},\"batch_processing\":false,\"distributed_equally\":false,\"processor_alias_name\":null,\"refuse_reviewers_mode\":0},\"metadata\":{\"position\":{\"x\":391,\"y\":102}},\"alias_name\":\"\",\"fields\":[{\"id\":7957,\"title\":\"输入一个测试文本\",\"description\":null,\"type\":\"Field::TextField\",\"position\":0,\"validations\":[],\"other_option\":null,\"visibility\":\"public_visibility\",\"marked\":false,\"settings\":{\"layout\":\"block\",\"char_size_limit_settings\":{},\"other_option_settings\":{\"limit\":{}},\"length_limit\":{},\"limit_settings\":{},\"enable_wechat_scan\":false},\"detail_id\":null,\"identity_key\":\"f3fbe5b4bdbd4019a622a5bc3e4ee915\",\"data\":{}}]},{\"id\":7182,\"name\":\"机器节点\",\"type\":\"YetAnotherWorkflow::Vertex::Normal\",\"field_ids\":[],\"operations\":{\"route\":{\"name\":\"选路\",\"enabled\":true},\"refuse\":{\"name\":\"回退\",\"enabled\":true},\"approve\":{\"name\":\"通过\",\"enabled\":true},\"comment\":{\"name\":\"处理意见\",\"enabled\":true},\"transfer\":{\"name\":\"转交\",\"enabled\":true},\"esignature\":{\"name\":\"电子签字\",\"enabled\":false}},\"settings\":{\"carbon_copy\":{\"enable_manual\":false},\"refuse_mode\":1,\"assign_manually\":0,\"comment_present\":{\"refuse\":false,\"approve\":false,\"transfer\":false},\"batch_processing\":false,\"duration_threshold\":{\"value\":null,\"enable_delay\":false,\"business_time\":{\"final\":\"17:00\",\"initial\":\"09:00\",\"workday\":true},\"enable_manual\":false,\"enable_business_time\":false},\"distributed_equally\":false,\"processor_alias_name\":null,\"refuse_reviewers_mode\":0},\"metadata\":{\"position\":{\"x\":391,\"y\":262.7109375}},\"alias_name\":\"\",\"fields\":[]},{\"id\":7178,\"name\":\"结束节点\",\"type\":\"YetAnotherWorkflow::Vertex::Final\",\"field_ids\":[],\"operations\":{},\"settings\":{},\"metadata\":{\"position\":{\"x\":391,\"y\":448}},\"alias_name\":\"\",\"fields\":[]}],\"edges\":[{\"id\":5186,\"from_vertex_id\":7177,\"to_vertex_id\":7182},{\"id\":5187,\"from_vertex_id\":7182,\"to_vertex_id\":7178}]}")
  }
]
```

