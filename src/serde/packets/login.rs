pub mod serverbound {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct LoginStart {
        pub name: String,
    }
}

pub mod clientbound {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct LoginSuccess<'a> {
        pub uuid: &'a uuid::Uuid,
        pub username: String,
    }
}
