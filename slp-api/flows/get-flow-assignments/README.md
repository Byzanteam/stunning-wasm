# get flow assignments

[GET /api/v4/yaw/journeys/:journey_id/assignments](https://byzanteam.github.io/skylark-v4-api-doc/#237874ebb3)

|参数|类型|描述|
|:--:|:--:|:--:|
|base url|String|基础url(test environment or develop environment)| 
|journey id|Integer|流程记录ID|
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
    "value" => 2883
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
        "assignee_id" => 73,
        "category" => "proposed",
        "created_at" => "2020-09-25T10:55:09.182+08:00",
        "current_duration_threshold" => nil,
        "hourglasses" => [],
        "id" => 6453,
        "journey_id" => 1799,
        "operation_data" => %{
          "next_vertex_id" => 5232,
          "operation" => "propose"
        },
        "read" => false,
        "response" => %{
          "cached_values" => %{
            "6911" => %{
              "exported_value" => ["234"],
              "text_value" => ["234"],
              "value" => ["234"]
            },
            "6913" => %{
              "exported_value" => [nil],
              "text_value" => [nil],
              "value" => [nil]
            }
          },
          "created_at" => "2020-09-16T16:39:41.453+08:00",
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 6913,
              "group_id" => nil,
              "id" => 1139265,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => nil,
              "value_id" => nil
            },
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 6911,
              "group_id" => nil,
              "id" => 1139260,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "234",
              "value_id" => nil
            }
          ],
          "id" => 76327,
          "involved_organization_ids" => [],
          "involved_user_ids" => [],
          "mapped_values" => %{
            "3893f09b02984460abd5dbcb4e9d63f6" => %{
              "exported_value" => [nil],
              "text_value" => [nil],
              "value" => [nil]
            },
            "bd08099e76a647a490a7a246d8df524f" => %{
              "exported_value" => ["234"],
              "text_value" => ["234"],
              "value" => ["234"]
            }
          },
          "updated_at" => "2020-09-25T10:56:26.672+08:00",
          "user" => %{
            "created_at" => "2016-11-30T16:27:41.452+08:00",
            "headimgurl" => "https://thirdwx.qlogo.cn/mmopen/vi_32/AxXZVEcytSkEu0BDLAXInM95oLCIkEhEBEaydMLDSUKdDn8XovKYGLfJibcZ1oAniaBOMsEs7fe4sgVpNnSQicpZA/96",
            "id" => 73,
            "identifier" => "18980807092",
            "name" => "苏凯",
            "nickname" => "招呼不到patrick",
            "openid" => "oXDm5s1iCl6KTAh8qxCmZYHhErdg",
            "organizations" => [
              %{
                "created_at" => "2020-08-04T20:50:21.070+08:00",
                "description" => "<p>这是测试用法111</p>",
                "id" => 594,
                "name" => "某某测试组"
              },
              %{
                "created_at" => "2020-11-11T10:47:08.441+08:00",
                "description" => nil,
                "id" => 598,
                "name" => "某某测试111"
              },
              %{
                "created_at" => "2022-06-14T15:36:42.148+08:00",
                "description" => "",
                "id" => 619,
                "name" => "22秋"
              },
              %{
                "created_at" => "2022-06-14T15:37:03.507+08:00",
                "description" => "",
                "id" => 621,
                "name" => "英语系"
              },
              %{
                "created_at" => "2022-06-14T15:37:23.945+08:00",
                "description" => "",
                "id" => 622,
                "name" => "产品"
              }
            ],
            "phone" => "18980807092",
            "profile_submission" => %{
              "entries" => [
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
                  "id" => 1091894,
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
            "updated_at" => "2022-11-01T10:32:57.254+08:00"
          }
        },
        "status" => "finished",
        "updated_at" => "2020-09-25T10:56:26.616+08:00",
        "vertex_id" => 5225
      },
      %{
        "assignee_id" => 719,
        "category" => "processed",
        "created_at" => "2020-09-25T10:55:09.413+08:00",
        "current_duration_threshold" => nil,
        "hourglasses" => [],
        "id" => 6479,
        "journey_id" => 1799,
        "operation_data" => %{
          "comment" => "",
          "next_vertex_id" => 5231,
          "operation" => "approve"
        },
        "read" => true,
        "response" => %{
          "cached_values" => %{},
          "created_at" => "2020-09-25T10:55:09.396+08:00",
          "entries" => [],
          "id" => 76364,
          "involved_organization_ids" => [],
          "involved_user_ids" => [],
          "mapped_values" => %{},
          "updated_at" => "2023-02-07T18:23:28.617+08:00",
          "user" => %{
            "created_at" => "2020-03-02T16:46:45.346+08:00",
            "headimgurl" => "http://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTLCfjRHjcmHkvEaGALy6eGN1abUfSsKeIib5jcpYM8iaVhjX0msx0iafd1XY8IZum6bDCqHiagTLYYB6Q/96",
            "id" => 719,
            "identifier" => "18030628377",
            "name" => "赵鑫",
            "nickname" => "枢笙",
            "openid" => "oXDm5s1iTSxFrkkGQk0F1qr2X1a4",
            "organizations" => [],
            "phone" => "18030628377",
            "sex" => 0,
            "tags" => [],
            "updated_at" => "2022-06-14T15:35:40.981+08:00"
          }
        },
        "status" => "approved",
        "updated_at" => "2020-09-25T10:55:41.986+08:00",
        "vertex_id" => 5232
      },
      %{
        "assignee_id" => 73,
        "category" => "processed",
        "created_at" => "2020-09-25T10:55:42.279+08:00",
        "current_duration_threshold" => nil,
        "hourglasses" => [],
        "id" => 6480,
        "journey_id" => 1799,
        "operation_data" => %{
          "comment" => "",
          "next_vertex_id" => 5234,
          "operation" => "approve"
        },
        "read" => true,
        "response" => %{
          "cached_values" => %{},
          "created_at" => "2020-09-25T10:55:42.260+08:00",
          "entries" => [],
          "id" => 76365,
          "involved_organization_ids" => [],
          "involved_user_ids" => [],
          "mapped_values" => %{},
          "updated_at" => "2023-02-07T18:23:28.635+08:00",
          "user" => %{
            "created_at" => "2016-11-30T16:27:41.452+08:00",
            "headimgurl" => "https://thirdwx.qlogo.cn/mmopen/vi_32/AxXZVEcytSkEu0BDLAXInM95oLCIkEhEBEaydMLDSUKdDn8XovKYGLfJibcZ1oAniaBOMsEs7fe4sgVpNnSQicpZA/96",
            "id" => 73,
            "identifier" => "18980807092",
            "name" => "苏凯",
            "nickname" => "招呼不到patrick",
            "openid" => "oXDm5s1iCl6KTAh8qxCmZYHhErdg",
            "organizations" => [
              %{
                "created_at" => "2020-08-04T20:50:21.070+08:00",
                "description" => "<p>这是测试用法111</p>",
                "id" => 594,
                "name" => "某某测试组"
              },
              %{
                "created_at" => "2020-11-11T10:47:08.441+08:00",
                "description" => nil,
                "id" => 598,
                "name" => "某某测试111"
              },
              %{
                "created_at" => "2022-06-14T15:36:42.148+08:00",
                "description" => "",
                "id" => 619,
                "name" => "22秋"
              },
              %{
                "created_at" => "2022-06-14T15:37:03.507+08:00",
                "description" => "",
                "id" => 621,
                "name" => "英语系"
              },
              %{
                "created_at" => "2022-06-14T15:37:23.945+08:00",
                "description" => "",
                "id" => 622,
                "name" => "产品"
              }
            ],
            "phone" => "18980807092",
            "profile_submission" => %{
              "entries" => [
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
                  "id" => 1091894,
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
            "updated_at" => "2022-11-01T10:32:57.254+08:00"
          }
        },
        "status" => "approved",
        "updated_at" => "2020-09-25T10:56:04.211+08:00",
        "vertex_id" => 5231
      },
      %{
        "assignee_id" => 719,
        "category" => "processed",
        "created_at" => "2020-09-25T10:56:04.370+08:00",
        "current_duration_threshold" => nil,
        "hourglasses" => [],
        "id" => 6481,
        "journey_id" => 1799,
        "operation_data" => %{
          "comment" => "",
          "next_vertex_id" => 5226,
          "operation" => "approve"
        },
        "read" => true,
        "response" => %{
          "cached_values" => %{},
          "created_at" => "2020-09-25T10:56:04.359+08:00",
          "entries" => [],
          "id" => 76366,
          "involved_organization_ids" => [],
          "involved_user_ids" => [],
          "mapped_values" => %{},
          "updated_at" => "2023-02-07T18:23:28.658+08:00",
          "user" => %{
            "created_at" => "2020-03-02T16:46:45.346+08:00",
            "headimgurl" => "http://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTLCfjRHjcmHkvEaGALy6eGN1abUfSsKeIib5jcpYM8iaVhjX0msx0iafd1XY8IZum6bDCqHiagTLYYB6Q/96",
            "id" => 719,
            "identifier" => "18030628377",
            "name" => "赵鑫",
            "nickname" => "枢笙",
            "openid" => "oXDm5s1iTSxFrkkGQk0F1qr2X1a4",
            "organizations" => [],
            "phone" => "18030628377",
            "sex" => 0,
            "tags" => [],
            "updated_at" => "2022-06-14T15:35:40.981+08:00"
          }
        },
        "status" => "approved",
        "updated_at" => "2020-09-25T10:56:26.383+08:00",
        "vertex_id" => 5234
      }
    ])
  }
]
```
