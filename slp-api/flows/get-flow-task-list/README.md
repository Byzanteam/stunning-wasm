# get flow task list

[GET /api/v4/yaw/flows/:id/journeys](https://byzanteam.github.io/skylark-v4-api-doc/#67b5696cb4)

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
    "value" => Jason.decode!("[{\"id\":2860,\"sn\":\"179920230105111556000001\",\"status\":\"processing\",\"current_duration_threshold\":null,\"flow_id\":1799,\"created_at\":\"2023-01-05T11:20:09.545+08:00\",\"updated_at\":\"2023-01-05T11:20:09.545+08:00\",\"current_vertex_id\":7182,\"reviewer_vertex_ids\":[7177],\"journey_url\":\"https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2860\",\"response\":{\"id\":81199,\"cached_values\":{\"7957\":{\"value\":[\"wasm-test\"],\"text_value\":[\"wasm-test\"],\"exported_value\":[\"wasm-test\"]}},\"mapped_values\":{\"f3fbe5b4bdbd4019a622a5bc3e4ee915\":{\"value\":[\"wasm-test\"],\"text_value\":[\"wasm-test\"],\"exported_value\":[\"wasm-test\"]}},\"entries\":[{\"id\":1143197,\"field_id\":7957,\"option_id\":null,\"value\":\"wasm-test\",\"choice_id\":null,\"value_id\":null,\"latitude\":null,\"longitude\":null,\"group_id\":null,\"detail_id\":null}]},\"user\":{\"id\":79,\"name\":\"樊翔宇\",\"nickname\":\"k k\",\"sex\":0,\"phone\":\"18828072450\",\"identifier\":\"18828072450\",\"openid\":\"oXDm5s2v7fnJ2Mut-UiiHtCEIb6Q\",\"created_at\":\"2017-02-22T13:56:54.125+08:00\",\"updated_at\":\"2022-11-21T14:29:26.011+08:00\",\"headimgurl\":\"https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTKFLWWPT1sSVywib8qpNNfLjMOliblYqa105ibCOKGvzwRV0vAEAlGLcwXBliboaad9udfsHcl7SKIfOw/96\"}},{\"id\":2861,\"sn\":\"179920230105115103000002\",\"status\":\"processing\",\"current_duration_threshold\":null,\"flow_id\":1799,\"created_at\":\"2023-01-05T14:08:52.764+08:00\",\"updated_at\":\"2023-01-05T14:08:52.764+08:00\",\"current_vertex_id\":7182,\"reviewer_vertex_ids\":[7177],\"journey_url\":\"https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2861\",\"response\":{\"id\":81202,\"cached_values\":{\"7957\":{\"value\":[\"webassbley test\"],\"text_value\":[\"webassbley test\"],\"exported_value\":[\"webassbley test\"]}},\"mapped_values\":{\"f3fbe5b4bdbd4019a622a5bc3e4ee915\":{\"value\":[\"webassbley test\"],\"text_value\":[\"webassbley test\"],\"exported_value\":[\"webassbley test\"]}},\"entries\":[{\"id\":1143199,\"field_id\":7957,\"option_id\":null,\"value\":\"webassbley test\",\"choice_id\":null,\"value_id\":null,\"latitude\":null,\"longitude\":null,\"group_id\":null,\"detail_id\":null}]},\"user\":{\"id\":79,\"name\":\"樊翔宇\",\"nickname\":\"k k\",\"sex\":0,\"phone\":\"18828072450\",\"identifier\":\"18828072450\",\"openid\":\"oXDm5s2v7fnJ2Mut-UiiHtCEIb6Q\",\"created_at\":\"2017-02-22T13:56:54.125+08:00\",\"updated_at\":\"2022-11-21T14:29:26.011+08:00\",\"headimgurl\":\"https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTKFLWWPT1sSVywib8qpNNfLjMOliblYqa105ibCOKGvzwRV0vAEAlGLcwXBliboaad9udfsHcl7SKIfOw/96\"}},{\"id\":2862,\"sn\":\"179920230105141741000003\",\"status\":\"processing\",\"current_duration_threshold\":null,\"flow_id\":1799,\"created_at\":\"2023-01-31T15:28:05.686+08:00\",\"updated_at\":\"2023-01-31T15:28:05.686+08:00\",\"current_vertex_id\":7182,\"reviewer_vertex_ids\":[7177],\"journey_url\":\"https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2862\",\"response\":{\"id\":81205,\"cached_values\":{\"7957\":{\"value\":[\"2022-test\"],\"text_value\":[\"2022-test\"],\"exported_value\":[\"2022-test\"]}},\"mapped_values\":{\"f3fbe5b4bdbd4019a622a5bc3e4ee915\":{\"value\":[\"2022-test\"],\"text_value\":[\"2022-test\"],\"exported_value\":[\"2022-test\"]}},\"entries\":[{\"id\":1143201,\"field_id\":7957,\"option_id\":null,\"value\":\"2022-test\",\"choice_id\":null,\"value_id\":null,\"latitude\":null,\"longitude\":null,\"group_id\":null,\"detail_id\":null}]},\"user\":{\"id\":79,\"name\":\"樊翔宇\",\"nickname\":\"k k\",\"sex\":0,\"phone\":\"18828072450\",\"identifier\":\"18828072450\",\"openid\":\"oXDm5s2v7fnJ2Mut-UiiHtCEIb6Q\",\"created_at\":\"2017-02-22T13:56:54.125+08:00\",\"updated_at\":\"2022-11-21T14:29:26.011+08:00\",\"headimgurl\":\"https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTKFLWWPT1sSVywib8qpNNfLjMOliblYqa105ibCOKGvzwRV0vAEAlGLcwXBliboaad9udfsHcl7SKIfOw/96\"}},{\"id\":2864,\"sn\":\"179920230131153329000004\",\"status\":\"processing\",\"current_duration_threshold\":null,\"flow_id\":1799,\"created_at\":\"2023-01-31T15:35:07.557+08:00\",\"updated_at\":\"2023-01-31T15:35:07.557+08:00\",\"current_vertex_id\":7182,\"reviewer_vertex_ids\":[7177],\"journey_url\":\"https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2864\",\"response\":{\"id\":81211,\"cached_values\":{\"7957\":{\"value\":[\"this is test\"],\"text_value\":[\"this is test\"],\"exported_value\":[\"this is test\"]}},\"mapped_values\":{\"f3fbe5b4bdbd4019a62" <> ...)
  }
]
```
