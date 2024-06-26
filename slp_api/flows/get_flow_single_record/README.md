# get flow single record

[GET /api/v4/yaw/flows/:id/journeys/:journey_id](https://byzanteam.github.io/skylark-v4-api-doc/#08f537c772)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|flow id|Integer|流程ID|
|joerney id|Integer|流程记录ID|
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
    "value" => 2862
  }
]
```

## Outputs
```elixir
[
  %{
    "field_type" => "SINGLE_LINE_FIELD",
    "type" => "LITERAL",
    "value" => Jason.encode!(%{
      "created_at" => "2023-01-31T15:28:05.686+08:00",
      "current_duration_threshold" => nil,
      "current_vertex_id" => 7182,
      "flow_id" => 1799,
      "hourglasses" => [],
      "id" => 2862,
      "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2862",
      "response" => %{
        "cached_values" => %{
          "7957" => %{
            "exported_value" => ["2022-test"],
            "text_value" => ["2022-test"],
            "value" => ["2022-test"]
          }
        },
        "entries" => [
          %{
            "choice_id" => nil,
            "detail_id" => nil,
            "field_id" => 7957,
            "group_id" => nil,
            "id" => 1143201,
            "latitude" => nil,
            "longitude" => nil,
            "option_id" => nil,
            "value" => "2022-test",
            "value_id" => nil
          }
        ],
        "id" => 81205,
        "mapped_values" => %{
          "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
            "exported_value" => ["2022-test"],
            "text_value" => ["2022-test"],
            "value" => ["2022-test"]
          }
        }
      },
      "reviewer_vertex_ids" => [7177],
      "sn" => "179920230105141741000003",
      "status" => "processing",
      "updated_at" => "2023-01-31T15:28:05.686+08:00",
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
    })
  }
]
```
