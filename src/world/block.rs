pub enum Block {
    Air,
    Stone(StoneType),
}

pub enum StoneType {
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andensite,
    PolishedAndensite,
}
