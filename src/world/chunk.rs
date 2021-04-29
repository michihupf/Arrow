use super::block::Block;

pub struct Chunk {
    blocks: [[[Block; 16]; 16]; 16]
}
