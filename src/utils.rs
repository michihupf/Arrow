#[macro_export]
macro_rules! map {
    { $($tag:tt = $val:expr),* } => {
        {
            let mut map = std::collections::HashMap::new();

            $( map.insert($tag.to_string(), $val); )*

            map
        }
    };

    { $self:ident; $($tag:ident),* } => {
        {
            let mut map = std::collections::HashMap::new();

            $( map.insert(stringify!($tag).to_string(), $self.$tag); )*

            map
        }
    }
}

#[macro_export]
macro_rules! option_map {
    { $map:ident; $($tag:tt = $val:expr),* } => {
        {
            $( if $val.is_some() {
                $map.insert($tag.to_string(), $val.unwrap().into());
            })*
        }
    };
    { $self:ident; $map:ident; $($tag:ident),* } => {
        {
            $( if $self.$tag.is_some() {
                $map.insert(stringify!($tag).to_string(), $self.$tag.unwrap().into());
            })*
        }
    }
}
