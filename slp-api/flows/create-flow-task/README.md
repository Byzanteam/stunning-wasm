# create flow task

[POST /api/v4/yaw/flows/:id/journeys](https://byzanteam.github.io/skylark-v4-api-doc/#route)

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
    "value" => "{\"assignment\":{\"operation\":\"route\",\"response_attributes\":{\"entries_attributes\":[{\"field_id\":\"7957\",\"value\":\"121212test\"}]}},\"user_id\":79}"
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
      "assignment" => %{
        "after_submit_action" => "default",
        "after_submit_redirect_url" => nil,
        "category" => "proposed",
        "created_at" => "2023-02-08T14:36:05.891+08:00",
        "current_duration_threshold" => nil,
        "gid" => "gid://skylark/YetAnotherWorkflow::Assignment/9820",
        "id" => 9820,
        "journey" => %{
          "created_at" => "2023-02-08T14:36:05.873+08:00",
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
          "status" => "stashed",
          "updated_at" => "2023-02-08T14:36:05.873+08:00",
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
        "status" => "stashed",
        "updated_at" => "2023-02-08T14:36:05.920+08:00"
      },
      "next_graphs" => [
        %{
          "ancestry" => nil,
          "created_at" => "2023-01-05T11:03:12.451+08:00",
          "gid" => "gid://skylark/YetAnotherWorkflow::Graph/2189",
          "id" => 2189,
          "metadata" => %{},
          "name" => "主流程",
          "settings" => %{
            "duration_threshold" => %{
              "business_time" => %{
                "final" => "17:00",
                "initial" => "09:00",
                "workday" => true
              },
              "enable_business_time" => false,
              "enable_delay" => false,
              "enable_manual" => false,
              "value" => nil
            },
            "visibility" => "public"
          },
          "updated_at" => "2023-01-05T11:03:12.564+08:00"
        }
      ],
      "next_vertices" => [
        %{
          "automatic" => true,
          "created_at" => "2023-01-05T11:14:17.205+08:00",
          "fields" => [],
          "gid" => "gid://skylark/YetAnotherWorkflow::Vertex::Normal/7182",
          "graph_id" => 2189,
          "id" => 7182,
          "logic_settings" => %{},
          "metadata" => %{"position" => %{"x" => 391, "y" => 262.7109375}},
          "name" => "机器节点",
          "operations" => %{
            "approve" => %{"enabled" => true, "name" => "通过"},
            "comment" => %{"enabled" => true, "name" => "处理意见"},
            "esignature" => %{"enabled" => false, "name" => "电子签字"},
            "refuse" => %{"enabled" => true, "name" => "回退"},
            "route" => %{"enabled" => true, "name" => "选路"},
            "transfer" => %{"enabled" => true, "name" => "转交"}
          },
          "settings" => %{
            "assign_manually" => 0,
            "batch_processing" => false,
            "carbon_copy" => %{"enable_manual" => false},
            "comment_present" => %{
              "approve" => false,
              "refuse" => false,
              "transfer" => false
            },
            "distributed_equally" => false,
            "duration_threshold" => %{
              "business_time" => %{
                "final" => "17:00",
                "initial" => "09:00",
                "workday" => true
              },
              "enable_business_time" => false,
              "enable_delay" => false,
              "enable_manual" => false,
              "value" => nil
            },
            "processor_alias_name" => nil,
            "refuse_mode" => 1,
            "refuse_reviewers_mode" => 0
          },
          "type" => "YetAnotherWorkflow::Vertex::Normal",
          "updated_at" => "2023-01-05T11:14:17.260+08:00"
        }
      ]
    })
  }
]
```
