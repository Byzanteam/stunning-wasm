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
    "value" => Jason.encode!([
      %{
        "created_at" => "2023-01-05T11:20:09.545+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2860,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2860",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["wasm-test"],
              "text_value" => ["wasm-test"],
              "value" => ["wasm-test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143197,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "wasm-test",
              "value_id" => nil
            }
          ],
          "id" => 81199,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["wasm-test"],
              "text_value" => ["wasm-test"],
              "value" => ["wasm-test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230105111556000001",
        "status" => "processing",
        "updated_at" => "2023-01-05T11:20:09.545+08:00",
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
      },
      %{
        "created_at" => "2023-01-05T14:08:52.764+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2861,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2861",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["webassbley test"],
              "text_value" => ["webassbley test"],
              "value" => ["webassbley test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143199,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "webassbley test",
              "value_id" => nil
            }
          ],
          "id" => 81202,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["webassbley test"],
              "text_value" => ["webassbley test"],
              "value" => ["webassbley test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230105115103000002",
        "status" => "processing",
        "updated_at" => "2023-01-05T14:08:52.764+08:00",
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
      },
      %{
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
      },
      %{
        "created_at" => "2023-01-31T15:35:07.557+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2864,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2864",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["this is test"],
              "text_value" => ["this is test"],
              "value" => ["this is test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143203,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "this is test",
              "value_id" => nil
            }
          ],
          "id" => 81211,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["this is test"],
              "text_value" => ["this is test"],
              "value" => ["this is test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131153329000004",
        "status" => "processing",
        "updated_at" => "2023-01-31T15:35:07.557+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T15:36:24.551+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2865,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2865",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["value No relationship"],
              "text_value" => ["value No relationship"],
              "value" => ["value No relationship"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143205,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "value No relationship",
              "value_id" => nil
            }
          ],
          "id" => 81214,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["value No relationship"],
              "text_value" => ["value No relationship"],
              "value" => ["value No relationship"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131153608000005",
        "status" => "processing",
        "updated_at" => "2023-01-31T15:36:24.551+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T15:37:02.326+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2866,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2866",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["No relationship"],
              "text_value" => ["No relationship"],
              "value" => ["No relationship"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143207,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "No relationship",
              "value_id" => nil
            }
          ],
          "id" => 81217,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["No relationship"],
              "text_value" => ["No relationship"],
              "value" => ["No relationship"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131153647000006",
        "status" => "processing",
        "updated_at" => "2023-01-31T15:37:02.326+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T15:57:49.484+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2867,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2867",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["qwert"],
              "text_value" => ["qwert"],
              "value" => ["qwert"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143209,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "qwert",
              "value_id" => nil
            }
          ],
          "id" => 81220,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["qwert"],
              "text_value" => ["qwert"],
              "value" => ["qwert"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131153951000007",
        "status" => "processing",
        "updated_at" => "2023-01-31T15:57:49.484+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T16:14:37.640+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2868,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2868",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143211,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "121212test",
              "value_id" => nil
            }
          ],
          "id" => 81223,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131161431000008",
        "status" => "processing",
        "updated_at" => "2023-01-31T16:14:37.640+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T16:17:48.462+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2869,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2869",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143213,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "121212test",
              "value_id" => nil
            }
          ],
          "id" => 81226,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131161639000009",
        "status" => "processing",
        "updated_at" => "2023-01-31T16:17:48.462+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T16:18:09.757+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2870,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2870",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143215,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "121212test",
              "value_id" => nil
            }
          ],
          "id" => 81229,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131161805000010",
        "status" => "processing",
        "updated_at" => "2023-01-31T16:18:09.757+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T16:39:32.157+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2871,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2871",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143217,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "121212test",
              "value_id" => nil
            }
          ],
          "id" => 81232,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131163613000011",
        "status" => "processing",
        "updated_at" => "2023-01-31T16:39:32.157+08:00",
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
      },
      %{
        "created_at" => "2023-01-31T16:52:24.389+08:00",
        "current_duration_threshold" => nil,
        "current_vertex_id" => 7182,
        "flow_id" => 1799,
        "hourglasses" => [],
        "id" => 2872,
        "journey_url" => "https://beta.skylarkly.com/namespaces/1/yet_another_workflow/journeys/2872",
        "response" => %{
          "cached_values" => %{
            "7957" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          },
          "entries" => [
            %{
              "choice_id" => nil,
              "detail_id" => nil,
              "field_id" => 7957,
              "group_id" => nil,
              "id" => 1143219,
              "latitude" => nil,
              "longitude" => nil,
              "option_id" => nil,
              "value" => "121212test",
              "value_id" => nil
            }
          ],
          "id" => 81235,
          "mapped_values" => %{
            "f3fbe5b4bdbd4019a622a5bc3e4ee915" => %{
              "exported_value" => ["121212test"],
              "text_value" => ["121212test"],
              "value" => ["121212test"]
            }
          }
        },
        "reviewer_vertex_ids" => [7177],
        "sn" => "179920230131163935000012",
        "status" => "processing",
        "updated_at" => "2023-01-31T16:52:24.389+08:00",
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
