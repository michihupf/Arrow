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
    pub struct LoginSuccess {
        pub uuid: String,
        pub username: String,
    }
}
