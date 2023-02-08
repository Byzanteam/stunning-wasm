# initiate flow task

[POST /api/v4/yaw/flows/:id/journeys](https://byzanteam.github.io/skylark-v4-api-doc/#propose)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|authorization token|String|授权令牌|
|content type|String|声明为json|
|flow id|Integer|流程ID|
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
    "value" => 1799
  },
  %{
    "field_type" => "single_line_field",
    "type" => "literal",
    "value" => "{\"assignment\":{\"operation\":\"propose\",\"next_vertex_id\":7182},\"user_id\":79}" 
  },
]
```
## Outputs
```elixir
[
  %{
    "field_type" => "SINGLE_LINE_FIELD",
    "type" => "LITERAL",
    "value" => Jason.encode!(%{
      "after_submit_action" => "default",
      "after_submit_redirect_url" => nil,
      "category" => "proposed",
      "created_at" => "2023-02-08T15:28:34.353+08:00",
      "current_duration_threshold" => nil,
      "gid" => "gid://skylark/YetAnotherWorkflow::Assignment/9820",
      "id" => 9820,
      "journey" => %{
        "created_at" => "2023-02-08T15:28:34.353+08:00",
        "current_duration_threshold" => nil,
        "custom_sn" => nil,
        "gid" => "gid://skylark/YetAnotherWorkflow::Journey/2881",
        "id" => 2881,
        "pretty_current_duration_threshold" => nil,
        "response" => %{
          "created_at" => "2023-02-08T14:36:05.865+08:00",
          "entries" => [],
          "id" => 81265
        },
        "sn" => "179920230208143605000014",
        "status" => "processing",
        "updated_at" => "2023-02-08T15:28:34.353+08:00",
        "user" => %{
          "gid" => "gid://skylark/User/79",
          "headimgurl" => "https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTKFLWWPT1sSVywib8qpNNfLjMOliblYqa105ibCOKGvzwRV0vAEAlGLcwXBliboaad9udfsHcl7SKIfOw",
          "id" => 79,
          "identifier" => "18828072450",
          "imported_alias" => "樊翔宇",
          "name" => "樊翔宇",
          "nickname" => "k k",
          "openid" => "oXDm5s2v7fnJ2Mut-UiiHtCEIb6Q",
          "phone" => "18828072450",
          "qq" => nil
        }
      },
      "pretty_current_duration_threshold" => nil,
      "read" => true,
      "response" => %{
        "created_at" => "2023-02-08T14:36:05.887+08:00",
        "entries" => [
          %{
            "choice_id" => nil,
            "detail_id" => nil,
            "field_id" => 7957,
            "group_id" => nil,
            "id" => 1143236,
            "latitude" => nil,
            "longitude" => nil,
            "option_id" => nil,
            "value" => "121212test",
            "value_id" => nil
          }
        ],
        "id" => 81266
      },
      "status" => "processing",
      "updated_at" => "2023-02-08T15:28:34.396+08:00"
    })
  }
]
```
