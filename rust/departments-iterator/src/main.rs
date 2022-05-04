#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType, field_value::UserBoundaryFieldValue,
        value::user_boundary::UserBoundary,
    },
    value_presenter::{literal::LiteralValuePresenter, ValuePresenter},
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let user_boundary_vp = inputs.get(0).unwrap();

    let (current, remaining): (UserBoundaryFieldValue, UserBoundaryFieldValue) =
        match user_boundary_vp {
            ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(
                UserBoundaryFieldValue::Value(user_boundary),
            )) => iterate_over_departments(user_boundary),
            ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(
                UserBoundaryFieldValue::Nil,
            )) => (UserBoundaryFieldValue::Nil, UserBoundaryFieldValue::Nil),
            _ => unreachable!(),
        };

    Outputs::build(vec![
        ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(current)),
        ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(remaining)),
    ])
}

fn iterate_over_departments(
    user_boundary: &UserBoundary,
) -> (UserBoundaryFieldValue, UserBoundaryFieldValue) {
    let current: Option<UserBoundary>;
    let remaining: Option<UserBoundary>;

    if let Some((first, uuids)) = user_boundary.simple_department_uuids.split_first() {
        current = Some(UserBoundary {
            user_uuids: vec![],
            simple_department_uuids: vec![first.clone()],
            penetrating_department_uuids: vec![],
        });
        remaining = Some(UserBoundary {
            user_uuids: vec![],
            simple_department_uuids: uuids.to_vec(),
            ..user_boundary.clone()
        });
    } else if let Some((first, uuids)) = user_boundary.penetrating_department_uuids.split_first() {
        current = Some(UserBoundary {
            user_uuids: vec![],
            simple_department_uuids: vec![],
            penetrating_department_uuids: vec![first.clone()],
        });
        remaining = Some(UserBoundary {
            user_uuids: vec![],
            penetrating_department_uuids: uuids.to_vec(),
            ..user_boundary.clone()
        });
    } else {
        current = None;
        remaining = Some(UserBoundary {
            user_uuids: vec![],
            ..user_boundary.clone()
        });
    }

    (
        current
            .and_then(|ub| {
                if ub.is_empty() {
                    Some(UserBoundaryFieldValue::Nil)
                } else {
                    Some(UserBoundaryFieldValue::Value(ub))
                }
            })
            .unwrap_or(UserBoundaryFieldValue::Nil),
        remaining
            .and_then(|ub| {
                if ub.is_empty() {
                    Some(UserBoundaryFieldValue::Nil)
                } else {
                    Some(UserBoundaryFieldValue::Value(ub))
                }
            })
            .unwrap_or(UserBoundaryFieldValue::Nil),
    )
}

program!(entrypoint, vec![FieldType::UserBoundaryField]);
