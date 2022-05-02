#![no_main]

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{field_type::FieldType, value_type::UserBoundary},
    value_presenter::{literal::LiteralValuePresenter, ValuePresenter},
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let user_boundary_vp = inputs.get(0).unwrap();

    let (current, remaining): (Option<UserBoundary>, Option<UserBoundary>) = match user_boundary_vp
    {
        ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(Some(user_boundary))) => {
            iterate_over_departments(user_boundary)
        }
        ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(None)) => (None, None),
        _ => unreachable!(),
    };

    Outputs::build(vec![
        ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(current)),
        ValuePresenter::Literal(LiteralValuePresenter::UserBoundaryField(remaining)),
    ])
}

fn iterate_over_departments(
    user_boundary: &UserBoundary,
) -> (Option<UserBoundary>, Option<UserBoundary>) {
    let current: Option<UserBoundary>;
    let remaining: Option<UserBoundary>;

    if let Some((first, uuids)) = user_boundary.simple_department_uuids.split_first() {
        current = Some(UserBoundary {
            simple_department_uuids: vec![first.clone()],
            ..user_boundary.clone()
        });
        remaining = Some(UserBoundary {
            simple_department_uuids: uuids.to_vec(),
            ..user_boundary.clone()
        });
    } else if let Some((first, uuids)) = user_boundary.penetrating_department_uuids.split_first() {
        current = Some(UserBoundary {
            penetrating_department_uuids: vec![first.clone()],
            ..user_boundary.clone()
        });
        remaining = Some(UserBoundary {
            penetrating_department_uuids: uuids.to_vec(),
            ..user_boundary.clone()
        });
    } else {
        current = None;
        remaining = Some(user_boundary.clone());
    }

    (
        current.and_then(|ub| if ub.is_empty() { None } else { Some(ub) }),
        remaining.and_then(|ub| if ub.is_empty() { None } else { Some(ub) }),
    )
}

program!(entrypoint, vec![FieldType::UserBoundaryField]);
