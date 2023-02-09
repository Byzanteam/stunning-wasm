# get user initiated flow

[GET /api/v4/yaw/flows/:flow_id/journeys/proposed_journeys](https://byzanteam.github.io/skylark-v4-api-doc/#5c6b398482)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|flow id|Integer|流程ID|
|user id|Integer|用户ID|
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
    "value" => 1803
  },
  %{
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 79
  }
]
```

## Outputs
```elixir
[
  %{
    "field_type" => "SINGLE_LINE_FIELD",
    "type" => "LITERAL",
    "value" => Jason.encode!([
      %{
        "created_at" => "2023-02-08T16:48:49.221+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7203,
        "flow_id" => 1803,
        "hourglasses" => [],
        "id" => 2883,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2883",
        "response" => %{
          "cached_values" => %{},
          "entries" => [],
          "id" => 81270,
          "mapped_values" => %{}
        },
        "reviewer_vertex_ids" => [7196],
        "sn" => "180320230208164809000001",
        "status" => "processing",
        "updated_at" => "2023-02-08T16:48:49.221+08:00",
        "user" => %{
          "created_at" => "2017-02-22T13:56:54.125+08:00",
          "headimgurl" => "https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTKFLWWPT1sSVywib8qpNNfLjMOliblYqa105ibCOKGvzwRV0vAEAlGLcwXBliboaad9udfsHcl7SKIfOw/96",
          "id" => 79,
          "identifier" => "18828072450",
          "name" => "樊翔宇",
          "nickname" => "k k",
          "openid" => "oXDm5s2v7fnJ2Mut-UiiHtCEIb6Q",
          "phone" => "18828072450",
          "sex" => 0,
          "updated_at" => "2022-11-21T14:29:26.011+08:00"
        }
      }
    ])
  }
]
```
