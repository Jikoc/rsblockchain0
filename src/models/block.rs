use super::blockchain::Blockchain;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    //块的索引
    pub index: usize,
    //块的生成时间
    pub time: String,

    //块的工作证明
    pub proof_of_work: u64,
    //上一个块的哈希
    pub previous_hash: String,
    //当前块的哈希
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String) -> Self {
        let mut block = Block {
            index: index as usize,
            time: cn_time(),
            proof_of_work: u64::default(),
            previous_hash: previous_hash.clone(),
            hash: String::default(),
        };
        block
    }
    //导入块数据，序列化，计算块的哈希
    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        let serialized_block_data = serde_json::to_string(&block_data).expect("未能完成块的序列化");
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    //挖区块哈希值
    pub fn mine(&mut self, blockchain: Blockchain) {
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work += 1;
                self.hash = self.calculate_hash();
            } else {
                break;
            }
        }
    }
}

//中国时间
pub fn cn_time() -> String {
    let offset = FixedOffset::east(8 * 3600);
    let utc_time = Utc::now();
    let local_time = utc_time.with_timezone(&offset);
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}