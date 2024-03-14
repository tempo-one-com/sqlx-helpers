use std::{collections::HashMap, fmt::Debug, hash::Hash};

use sqlx::Error;

#[derive(Clone, Debug)]
pub struct OneToMany<A, B> {
    store: HashMap<A, Vec<B>>,
}

impl<A, B> OneToMany<A, B> {
    pub fn extract<T>(
        one: impl Fn(T) -> A,
        many: impl Fn(T) -> Result<B, Error>,
        rows: Vec<T>,
    ) -> Self
    where
        A: From<T> + Debug + Eq + PartialEq + Hash + Clone,
        B: TryFrom<T> + Debug + Clone,
        T: Clone,
    {
        let mut my_map = HashMap::<A, Vec<B>>::with_capacity(100);

        for row in rows {
            let a = one(row.clone());
            let b_opt = many(row.clone());

            match b_opt {
                Ok(b) => my_map.entry(a).or_default().push(b),
                _ => {
                    let _ = my_map.entry(a).or_default();
                }
            };
        }

        Self { store: my_map }
    }

    pub fn combine(&self, combinator: impl Fn(A, Vec<B>) -> A) -> Vec<A>
    where
        A: Clone,
        B: Clone,
    {
        self.store
            .clone()
            .into_iter()
            .map(|x| combinator(x.0, x.1))
            .collect::<Vec<_>>()
    }

    pub fn as_vec(&self) -> Vec<(A, Vec<B>)>
    where
        A: Clone,
        B: Clone,
    {
        self.store
            .clone()
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect::<Vec<_>>()
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
        pub is_admin: Option<bool>,
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
    fn test_1() {
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
            TeamUser {
                team_id: 3,
                name: Some("C Team".to_owned()),
                user_id: Some(20),
                username: Some("C30".to_owned()),
                ..Default::default()
            },
        ];
        let receps = OneToMany::extract(TeamDto::from, UserDto::try_from, rows)
            .combine(|r: TeamDto, e: Vec<UserDto>| TeamDto { users: e, ..r });
        dbg!(&receps);
        assert_eq!(receps.len(), 3);
        assert_eq!(
            get_by_id(&receps, 1).unwrap().user_codes(),
            vec!["A10", "A11"]
        );
        assert_eq!(get_by_id(&receps, 1).unwrap().id, 1);

        assert!(get_by_id(&receps, 2).unwrap().user_codes().is_empty());
        assert_eq!(get_by_id(&receps, 2).unwrap().id, 2);

        assert_eq!(get_by_id(&receps, 3).unwrap().user_codes(), vec!["C30"]);
        assert_eq!(get_by_id(&receps, 3).unwrap().id, 3);
    }
}
