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
    "field_type" => "numeric_field",
    "type" => "literal",
    "value" => 1799
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
      "edges" => [
        %{"from_vertex_id" => 7177, "id" => 5186, "to_vertex_id" => 7182},
        %{"from_vertex_id" => 7182, "id" => 5187, "to_vertex_id" => 7178}
      ],
      "external_settings" => %{},
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
      "html_body" => nil,
      "id" => 1799,
      "identity_vertify_type" => "off",
      "logic_settings" => %{
        "constraints" => %{},
        "controlled_field_ids" => [],
        "valid" => true
      },
      "title" => "wasm-test",
      "vertices" => [
        %{
          "alias_name" => "",
          "field_ids" => [],
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
          "metadata" => %{"position" => %{"x" => 391, "y" => 102}},
          "name" => "开始节点",
          "operations" => %{
            "cancel" => %{"enabled" => true, "name" => "撤销"},
            "comment" => %{"enabled" => true, "name" => "处理意见"},
            "esignature" => %{"enabled" => false, "name" => "电子签字"},
            "propose" => %{"enabled" => true, "name" => "发起"},
            "remind" => %{"enabled" => true, "name" => "催促"},
            "route" => %{"enabled" => true, "name" => "选路"},
            "stash" => %{"enabled" => true, "name" => "保存"}
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
            "processor_alias_name" => nil,
            "refuse_mode" => 1,
            "refuse_reviewers_mode" => 0
          },
          "type" => "YetAnotherWorkflow::Vertex::Initial"
        },
        %{
          "alias_name" => "",
          "field_ids" => [],
          "fields" => [],
          "id" => 7182,
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
          "type" => "YetAnotherWorkflow::Vertex::Normal"
        },
        %{
          "alias_name" => "",
          "field_ids" => [],
          "fields" => [],
          "id" => 7178,
          "metadata" => %{"position" => %{"x" => 391, "y" => 448}},
          "name" => "结束节点",
          "operations" => %{},
          "settings" => %{},
          "type" => "YetAnotherWorkflow::Vertex::Final"
        }
      ]
    }
  }
]
```
