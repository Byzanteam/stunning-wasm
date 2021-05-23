#![no_main]

use eetf::{Binary, List, Map, Term};
use std::io::Cursor;
use std::str;

extern "C" {
    pub fn hostcall_set_outputs(outputs_ptr: *const u8, outputs_len: usize);
}

#[derive(Debug, Clone)]
struct UUID(Vec<u8>);

#[derive(Debug, Clone)]
struct UserBoundary {
    simple_department_uuids: Vec<UUID>,
    penetrating_department_uuids: Vec<UUID>,
}

impl UserBoundary {
    pub fn empty() -> Self {
        UserBoundary {
            simple_department_uuids: vec![],
            penetrating_department_uuids: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.simple_department_uuids.len() == 0 && self.penetrating_department_uuids.len() == 0
    }
}

fn decode_string(term: &Term) -> Option<Vec<u8>> {
    match term {
        Term::Binary(binary) => Some(binary.bytes.clone()),
        _ => None,
    }
}

fn decode_uuids(term: &Term) -> Vec<UUID> {
    let mut vec: Vec<UUID> = Vec::new();

    if let Term::List(list) = term {
        list.elements
            .iter()
            .for_each(|item| match decode_string(item) {
                Some(x) => {
                    vec.push(UUID(x));
                }
                None => (),
            })
    }

    vec
}

impl From<&Map> for UserBoundary {
    fn from(map: &Map) -> Self {
        let mut simple_department_uuids: Vec<UUID> = vec![];
        let mut penetrating_department_uuids: Vec<UUID> = vec![];

        for (key_term, value_term) in map.entries.iter() {
            match decode_string(key_term) {
                None => (),
                Some(key) => match str::from_utf8(&key).ok() {
                    Some("simple_department_uuids") => {
                        simple_department_uuids = decode_uuids(value_term);
                    }
                    Some("penetrating_department_uuids") => {
                        penetrating_department_uuids = decode_uuids(value_term);
                    }
                    _ => (),
                },
            };
        }

        UserBoundary {
            simple_department_uuids: simple_department_uuids,
            penetrating_department_uuids: penetrating_department_uuids,
        }
    }
}

impl From<UUID> for Term {
    fn from(uuid: UUID) -> Self {
        Term::from(Binary::from(uuid.0))
    }
}

impl From<UserBoundary> for Term {
    fn from(user_boundary: UserBoundary) -> Self {
        if user_boundary.is_empty() {
            Term::from(List::nil())
        } else {
            let entries = vec![
                (
                    Term::from(Binary::from("simple_department_uuids".as_bytes())),
                    Term::from(List::from(
                        user_boundary
                            .simple_department_uuids
                            .into_iter()
                            .map(|uuid| Term::from(uuid))
                            .collect::<Vec<Term>>(),
                    )),
                ),
                (
                    Term::from(Binary::from("penetrating_department_uuids".as_bytes())),
                    Term::from(List::from(
                        user_boundary
                            .penetrating_department_uuids
                            .into_iter()
                            .map(|uuid| Term::from(uuid))
                            .collect::<Vec<Term>>(),
                    )),
                ),
                (
                    Term::from(Binary::from("user_uuids".as_bytes())),
                    Term::from(List::from(vec![])),
                ),
            ];

            Term::from(Map::from(entries))
        }
    }
}

#[no_mangle]
pub fn run(params: &str) {
    let bytes = params.as_bytes();
    let term = Term::decode(Cursor::new(&bytes)).ok();

    match term {
        Some(Term::List(list)) => match list.elements.first() {
            Some(Term::Map(user_boundary_map)) => {
                let (current, remaining) = interate(user_boundary_map);

                let outputs = List::from(vec![Term::from(current), Term::from(remaining)]);
                let term = Term::from(outputs);

                let mut buf = Vec::new();

                term.encode(&mut buf).unwrap();

                unsafe {
                    hostcall_set_outputs(buf.as_ptr(), buf.len());
                }
            }
            _ => (),
        },
        _ => (),
    }
}

fn interate(user_boundary_map: &Map) -> (UserBoundary, UserBoundary) {
    let user_boundary = UserBoundary::from(user_boundary_map);
    let simple_department_uuids = user_boundary.simple_department_uuids;
    let penetrating_department_uuids = user_boundary.penetrating_department_uuids;

    if let Some((first, uuids)) = simple_department_uuids.split_first() {
        (
            UserBoundary {
                simple_department_uuids: vec![UUID(first.0.clone())],
                penetrating_department_uuids: vec![],
            },
            UserBoundary {
                simple_department_uuids: uuids.to_vec(),
                penetrating_department_uuids: penetrating_department_uuids.to_vec(),
            },
        )
    } else if let Some((first, uuids)) = penetrating_department_uuids.split_first() {
        (
            UserBoundary {
                simple_department_uuids: vec![],
                penetrating_department_uuids: vec![UUID(first.0.clone())],
            },
            UserBoundary {
                simple_department_uuids: vec![],
                penetrating_department_uuids: uuids.to_vec(),
            },
        )
    } else {
        (UserBoundary::empty(), UserBoundary::empty())
    }
}
