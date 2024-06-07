use super::block::{cn_time, Block};
use chrono::prelude::*; //网络模块

type Blocks = Vec<Block>;

#[derive(Debug,Clone)]
pub struct Blockchain {
    //区块链中被添加的第一个块
    pub genesis_block: Block,
    //块存储在链上
    pub chain: Blocks,
    //验证区块所需要的最小工作量(难度)
    pub difficulty: usize,
}
impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        //创世区块
        let mut genesis_block = Block {
            index: 0,
            time: cn_time(),
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };
        //创建以创世区块为头的链
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());
        //创建区块链实例
        let mut blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty,
        };
        blockchain
    }
    //在区块链中添加新块
    pub fn add_block(&mut self) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].hash.clone(),
        );
        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        println!("新块添加到区块链中 -> {:#?}", new_block);
    }
}
