use std::{collections::HashMap, fmt::Debug, hash::Hash};

use sqlx::Error;

pub struct OneToMany;

//impl<A, B, C> OneToManyTrait<A, B, C> for OneToMany {}

impl OneToMany {
    pub fn extract<T, A, B>(
        rows: &[T],
        one: impl Fn(T) -> A,
        many: impl Fn(T) -> Result<B, Error>,
    ) -> Vec<(A, Vec<B>)>
    where
        A: From<T> + Debug + Eq + PartialEq + Hash + Clone,
        B: TryFrom<T> + Debug + Clone,
        T: Clone,
    {
        let mut items = HashMap::<A, Vec<B>>::with_capacity(500);

        for row in rows {
            let a = one(row.clone());
            let b_opt = many(row.clone());

            match b_opt {
                Ok(b) => items.entry(a).or_default().push(b),
                _ => {
                    let _ = items.entry(a).or_default();
                }
            };
        }

        items.into_iter().map(|x| (x.0, x.1)).collect::<Vec<_>>()
    }

    pub fn extract_from_ordered<T, A, B>(
        rows: &[T],
        one: impl Fn(T) -> A,
        many: impl Fn(T) -> Result<B, Error>,
    ) -> Vec<(A, Vec<B>)>
    where
        A: From<T> + Debug + Eq + PartialEq + Hash + Clone,
        B: TryFrom<T> + Debug + Clone,
        T: Clone,
    {
        let mut items: Vec<(A, Vec<B>)> = vec![];
        let mut current: Option<(A, Vec<B>)> = rows.first().map(|x| (one(x.clone()), vec![]));

        for row in rows {
            let a = one(row.clone());
            let b_opt = many(row.clone());

            match b_opt {
                Ok(b) => match current {
                    Some((curr, evts)) if curr == a => {
                        let mut v = evts;
                        v.push(b);
                        current = Some((curr, v));
                    }
                    Some((curr, evts)) => {
                        items.push((curr.clone(), evts.clone()));
                        current = Some((a, vec![b]))
                    }
                    None => current = Some((a, vec![b])),
                },
                _ => match current {
                    Some((curr, evts)) if curr != a => {
                        items.push((curr, evts));
                        current = Some((a, vec![]))
                    }
                    Some((_, _)) => {}
                    None => current = Some((a, vec![])),
                },
            };
        }

        if let Some(cur) = current {
            items.push(cur)
        }

        items
    }

    pub fn combine<A, B>(values: Vec<(A, Vec<B>)>, combinator: impl Fn(A, Vec<B>) -> A) -> Vec<A>
    where
        A: Clone,
        B: Clone,
    {
        values
            .into_iter()
            .map(|x| combinator(x.0, x.1))
            .collect::<Vec<_>>()
    }

    pub fn merge_one_to_manies<A, B, C>(
        first: Vec<(A, Vec<B>)>,
        second: Vec<(A, Vec<C>)>,
    ) -> Vec<(A, Vec<B>, Vec<C>)>
    where
        A: Debug + Eq + PartialEq + Hash + Clone,
        B: Debug + Clone,
        C: Debug + Clone,
    {
        let mut second_map: std::collections::HashMap<A, Vec<C>> = second.into_iter().collect();

        first
            .into_iter()
            .map(|(p, b_vec)| {
                let c_vec = second_map.remove(&p).unwrap_or_else(Vec::new);
                (p, b_vec, c_vec)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Error;

    use crate::one_to_many::OneToMany;

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct UserDto {
        pub id: i32,
        pub username: String,
        pub email: String,
    }

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct TeamDto {
        pub id: i32,
        pub name: String,
        pub users: Vec<UserDto>,
    }

    impl TeamDto {
        fn user_codes(&self) -> Vec<String> {
            self.users.clone().into_iter().map(|x| x.username).collect()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct TeamUser {
        pub team_id: i32,
        pub name: Option<String>,
        pub user_id: Option<i32>,
        pub username: Option<String>,
        pub email: Option<String>,
    }

    impl From<TeamUser> for TeamDto {
        fn from(value: TeamUser) -> Self {
            Self {
                id: value.team_id,
                name: value.name.unwrap_or_default(),
                users: vec![],
            }
        }
    }

    impl TryFrom<TeamUser> for UserDto {
        type Error = Error;
        fn try_from(value: TeamUser) -> Result<Self, Error> {
            match value.user_id {
                Some(id) => Ok(Self {
                    id,
                    username: value.username.unwrap_or_default(),
                    email: value.email.unwrap_or_default(),
                }),
                None => Err(Error::ColumnNotFound("user_id".to_string())),
            }
        }
    }

    fn get_by_id(items: &[TeamDto], id: i32) -> Option<TeamDto> {
        items.iter().find(|x| x.id == id).cloned()
    }

    #[test]
    fn test_combine() {
        let rows = vec![
            TeamUser {
                team_id: 3,
                name: Some("C Team".to_owned()),
                user_id: Some(21),
                username: Some("C31".to_owned()),
                ..Default::default()
            },
            TeamUser {
                team_id: 1,
                name: Some("A Team".to_owned()),
                user_id: Some(10),
                username: Some("A10".to_owned()),
                ..Default::default()
            },
            TeamUser {
                team_id: 1,
                name: Some("A Team".to_owned()),
                user_id: Some(11),
                username: Some("A11".to_owned()),
                ..Default::default()
            },
            TeamUser {
                team_id: 2,
                name: Some("B Team".to_owned()),
                user_id: None,
                ..Default::default()
            },
            TeamUser {
                team_id: 3,
                name: Some("C Team".to_owned()),
                user_id: Some(20),
                username: Some("C30".to_owned()),
                ..Default::default()
            },
        ];
        let receps: Vec<(TeamDto, Vec<UserDto>)> =
            OneToMany::extract(&rows, TeamDto::from, UserDto::try_from);
        let receps = OneToMany::combine(receps, |r: TeamDto, e: Vec<UserDto>| TeamDto {
            users: e,
            ..r
        });
        assert_eq!(receps.len(), 3);
        assert_eq!(
            get_by_id(&receps, 1).unwrap().user_codes(),
            vec!["A10", "A11"]
        );
        assert_eq!(get_by_id(&receps, 1).unwrap().id, 1);

        assert!(get_by_id(&receps, 2).unwrap().user_codes().is_empty());
        assert_eq!(get_by_id(&receps, 2).unwrap().id, 2);

        assert_eq!(get_by_id(&receps, 3).unwrap().id, 3);
    }

    #[test]
    fn test_as_vec() {
        let rows = vec![
            TeamUser {
                team_id: 1,
                name: Some("A Team".to_owned()),
                user_id: Some(10),
                username: Some("A10".to_owned()),
                ..Default::default()
            },
            TeamUser {
                team_id: 1,
                name: Some("A Team".to_owned()),
                user_id: Some(11),
                username: Some("A11".to_owned()),
                ..Default::default()
            },
            TeamUser {
                team_id: 2,
                name: Some("B Team".to_owned()),
                user_id: None,
                ..Default::default()
            },
        ];
        let receps = OneToMany::extract_from_ordered(&rows, TeamDto::from, UserDto::try_from);

        assert_eq!(receps.len(), 2);
        assert_eq!(
            receps.iter().map(|x| x.0.id).collect::<Vec<_>>(),
            vec![1, 2]
        );
        if let Some((_, a_users)) = receps.iter().find(|(x, _)| x.id == 1).cloned() {
            assert_eq!(a_users.len(), 2);
        } else {
            panic!("test_as_vec");
        }
    }
}
