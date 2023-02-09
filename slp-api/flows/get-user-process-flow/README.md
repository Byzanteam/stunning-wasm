# get user process flow

[GET /api/v4/yaw/flows/user_assignments.json?user_id=:user_id&category=:category](https://byzanteam.github.io/skylark-v4-api-doc/#54c0a12995)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|user id|Integer|用户ID|
|category|String|category 固定为: proposed「我发起的」，processed 「由我处理」，cc「抄送我的」|
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
    "value" => 79
  },
  %{
    "field_type" => "single_line_field",
    "type" => "literal",
    "value" => "proposed"
  },
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
        "assignee_id" => 79,
        "category" => "proposed",
        "created_at" => "2023-02-08T16:48:49.221+08:00",
        "current_duration_threshold" => nil,
        "hourglasses" => [],
        "id" => 9823,
        "journey_id" => 2883,
        "operation_data" => %{
          "next_vertex_id" => 7203,
          "operation" => "propose"
        },
        "read" => true,
        "response" => %{
          "cached_values" => %{},
          "created_at" => "2023-02-08T16:48:09.907+08:00",
          "entries" => [],
          "id" => 81271,
          "involved_organization_ids" => [],
          "involved_user_ids" => [],
          "mapped_values" => %{},
          "updated_at" => "2023-02-09T15:27:57.010+08:00",
          "user" => %{
            "created_at" => "2017-02-22T13:56:54.125+08:00",
            "headimgurl" => "https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTKFLWWPT1sSVywib8qpNNfLjMOliblYqa105ibCOKGvzwRV0vAEAlGLcwXBliboaad9udfsHcl7SKIfOw/96",
            "id" => 79,
            "identifier" => "18828072450",
            "name" => "樊翔宇",
            "nickname" => "k k",
            "openid" => "oXDm5s2v7fnJ2Mut-UiiHtCEIb6Q",
            "organizations" => [
              %{
                "created_at" => "2022-06-14T15:36:42.148+08:00",
                "description" => "",
                "id" => 619,
                "name" => "22秋"
              }
            ],
            "phone" => "18828072450",
            "profile_submission" => %{
              "entries" => [
                %{
                  "choice_id" => nil,
                  "detail_id" => nil,
                  "field_id" => 6058,
                  "group_id" => nil,
                  "id" => 1143157,
                  "latitude" => nil,
                  "longitude" => nil,
                  "option_id" => nil,
                  "value" => "test",
                  "value_id" => nil
                },
                %{
                  "choice" => %{
                    "ancestry" => "7647/7648/7649",
                    "children_count" => 0,
                    "id" => 7650,
                    "locked" => true,
                    "name" => "桂溪街办",
                    "parent_id" => 7649
                  },
                  "choice_id" => 7650,
                  "detail_id" => nil,
                  "field_id" => 6035,
                  "group_id" => nil,
                  "id" => 1091241,
                  "latitude" => nil,
                  "longitude" => nil,
                  "option_id" => nil,
                  "value" => "四川省-成都市-高新区-桂溪街办",
                  "value_id" => nil
                }
              ]
            },
            "sex" => 0,
            "tags" => [],
            "updated_at" => "2022-11-21T14:29:26.011+08:00"
          }
        },
        "status" => "processing",
        "updated_at" => "2023-02-08T16:48:49.265+08:00",
        "vertex_id" => 7196
      },
    ])
  }
]
```

