# get flow single record

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
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 1799
  },
  %{
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 7177
  },
  %{
    "field_type" => "single_line_field",
    "type" => "literal",
    "value" => "b01110629541b3eb51697db5a05dd2388aed11a58c81a75e9c08347bc30a09e6:eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lc3BhY2VfaWQiOjF9.wj9V0ZVOOzSPuRYztizJL_5w0u8aJKb05Z73tEV_HuY"
  }
]
```

## Outputs
```elixir
[
  %{
    "field_type" => "SINGLE_LINE_FIELD",
    "type" => "LITERAL",
    "value" => %{
      "alias_name" => "",
      "fields" => [
        %{
          "data" => %{},
          "description" => nil,
          "detail_id" => nil,
          "id" => 7957,
          "identity_key" => "f3fbe5b4bdbd4019a622a5bc3e4ee915",
          "marked" => false,
          "other_option" => nil,
          "position" => 0,
          "settings" => %{
            "char_size_limit_settings" => %{},
            "enable_wechat_scan" => false,
            "layout" => "block",
            "length_limit" => %{},
            "limit_settings" => %{},
            "other_option_settings" => %{"limit" => %{}}
          },
          "title" => "输入一个测试文本",
          "type" => "Field::TextField",
          "validations" => [],
          "visibility" => "public_visibility"
        }
      ],
      "id" => 7177,
      "name" => "开始节点",
      "push_settings" => [
        %{
          "push_scope" => "review",
          "push_time" => "created",
          "push_type" => "Push::CustomMessage",
          "waiting" => 0
        },
        %{
          "push_scope" => "cc",
          "push_time" => "updated",
          "push_type" => "Push::CustomMessage",
          "waiting" => 0
        },
        %{
          "push_scope" => "review",
          "push_time" => "time_up",
          "push_type" => "Push::CustomMessage",
          "waiting" => 0
        },
        %{
          "push_scope" => "cc",
          "push_time" => "time_up",
          "push_type" => "Push::CustomMessage",
          "waiting" => 0
        }
      ],
      "type" => "YetAnotherWorkflow::Vertex::Initial",
      "user_boundary" => %{
        "append_count" => 0,
        "cached_organization_ids" => [],
        "cached_user_ids" => [],
        "created_at" => "2023-01-05T11:03:12.465+08:00",
        "extra_boundary" => %{"reviewer_vertex_ids" => []},
        "id" => 9936,
        "namespace_id" => 1,
        "only_binded" => false,
        "organization_ids" => [],
        "overlord_id" => 7177,
        "overlord_type" => "YetAnotherWorkflow::Vertex",
        "status" => "default",
        "type" => "UserBoundary::VertexReview",
        "updated_at" => "2023-01-05T11:14:17.276+08:00",
        "user_ids" => [],
        "user_tag_ids" => [],
        "user_tagged_type" => "intersection"
      }
    }
  }
]
```

