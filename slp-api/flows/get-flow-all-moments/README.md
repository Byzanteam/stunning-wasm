# get flow all moments

[GET /api/v4/yaw/journeys/:id/moments](https://byzanteam.github.io/skylark-v4-api-doc/#moment)

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
    "value" => [
      %{
        "assignment" => %{
          "assignee_id" => 73,
          "category" => "proposed",
          "created_at" => "2020-09-25T10:55:09.182+08:00",
          "current_duration_threshold" => nil,
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
            "id" => 76327,
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
            }
          },
          "status" => "finished",
          "updated_at" => "2020-09-25T10:56:26.616+08:00",
          "vertex_id" => 5225
        },
        "comment" => nil,
        "created_at" => "2020-09-25T10:55:09.254+08:00",
        "esignature" => nil,
        "id" => 5398,
        "journey_id" => 1799,
        "status" => "proposed",
        "updated_at" => "2020-09-25T10:55:09.254+08:00",
        "user" => %{
          "created_at" => "2016-11-30T16:27:41.452+08:00",
          "headimgurl" => "https://thirdwx.qlogo.cn/mmopen/vi_32/AxXZVEcytSkEu0BDLAXInM95oLCIkEhEBEaydMLDSUKdDn8XovKYGLfJibcZ1oAniaBOMsEs7fe4sgVpNnSQicpZA/96",
          "id" => 73,
          "identifier" => "18980807092",
          "name" => "苏凯",
          "nickname" => "招呼不到patrick",
          "openid" => "oXDm5s1iCl6KTAh8qxCmZYHhErdg",
          "phone" => "18980807092",
          "sex" => 0,
          "updated_at" => "2022-11-01T10:32:57.254+08:00"
        },
        "vertex_id" => 5225
      },
      %{
        "assignment" => %{
          "assignee_id" => 719,
          "category" => "processed",
          "created_at" => "2020-09-25T10:55:09.413+08:00",
          "current_duration_threshold" => nil,
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
            "id" => 76364,
            "mapped_values" => %{}
          },
          "status" => "approved",
          "updated_at" => "2020-09-25T10:55:41.986+08:00",
          "vertex_id" => 5232
        },
        "comment" => "",
        "created_at" => "2020-09-25T10:55:09.272+08:00",
        "esignature" => nil,
        "id" => 5399,
        "journey_id" => 1799,
        "status" => "approved",
        "updated_at" => "2020-09-25T10:55:41.999+08:00",
        "user" => %{
          "created_at" => "2020-03-02T16:46:45.346+08:00",
          "headimgurl" => "http://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTLCfjRHjcmHkvEaGALy6eGN1abUfSsKeIib5jcpYM8iaVhjX0msx0iafd1XY8IZum6bDCqHiagTLYYB6Q/96",
          "id" => 719,
          "identifier" => "18030628377",
          "name" => "赵鑫",
          "nickname" => "枢笙",
          "openid" => "oXDm5s1iTSxFrkkGQk0F1qr2X1a4",
          "phone" => "18030628377",
          "sex" => 0,
          "updated_at" => "2022-06-14T15:35:40.981+08:00"
        },
        "vertex_id" => 5232
      },
      %{
        "assignment" => %{
          "assignee_id" => 73,
          "category" => "processed",
          "created_at" => "2020-09-25T10:55:42.279+08:00",
          "current_duration_threshold" => nil,
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
            "id" => 76365,
            "mapped_values" => %{}
          },
          "status" => "approved",
          "updated_at" => "2020-09-25T10:56:04.211+08:00",
          "vertex_id" => 5231
        },
        "comment" => "",
        "created_at" => "2020-09-25T10:55:42.010+08:00",
        "esignature" => nil,
        "id" => 5400,
        "journey_id" => 1799,
        "status" => "approved",
        "updated_at" => "2020-09-25T10:56:04.224+08:00",
        "user" => %{
          "created_at" => "2016-11-30T16:27:41.452+08:00",
          "headimgurl" => "https://thirdwx.qlogo.cn/mmopen/vi_32/AxXZVEcytSkEu0BDLAXInM95oLCIkEhEBEaydMLDSUKdDn8XovKYGLfJibcZ1oAniaBOMsEs7fe4sgVpNnSQicpZA/96",
          "id" => 73,
          "identifier" => "18980807092",
          "name" => "苏凯",
          "nickname" => "招呼不到patrick",
          "openid" => "oXDm5s1iCl6KTAh8qxCmZYHhErdg",
          "phone" => "18980807092",
          "sex" => 0,
          "updated_at" => "2022-11-01T10:32:57.254+08:00"
        },
        "vertex_id" => 5231
      },
      %{
        "assignment" => %{
          "assignee_id" => 719,
          "category" => "processed",
          "created_at" => "2020-09-25T10:56:04.370+08:00",
          "current_duration_threshold" => nil,
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
            "id" => 76366,
            "mapped_values" => %{}
          },
          "status" => "approved",
          "updated_at" => "2020-09-25T10:56:26.383+08:00",
          "vertex_id" => 5234
        },
        "comment" => "",
        "created_at" => "2020-09-25T10:56:04.236+08:00",
        "esignature" => nil,
        "id" => 5401,
        "journey_id" => 1799,
        "status" => "approved",
        "updated_at" => "2020-09-25T10:56:26.394+08:00",
        "user" => %{
          "created_at" => "2020-03-02T16:46:45.346+08:00",
          "headimgurl" => "http://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTLCfjRHjcmHkvEaGALy6eGN1abUfSsKeIib5jcpYM8iaVhjX0msx0iafd1XY8IZum6bDCqHiagTLYYB6Q/96",
          "id" => 719,
          "identifier" => "18030628377",
          "name" => "赵鑫",
          "nickname" => "枢笙",
          "openid" => "oXDm5s1iTSxFrkkGQk0F1qr2X1a4",
          "phone" => "18030628377",
          "sex" => 0,
          "updated_at" => "2022-06-14T15:35:40.981+08:00"
        },
        "vertex_id" => 5234
      }
    ]
  }
]
```
