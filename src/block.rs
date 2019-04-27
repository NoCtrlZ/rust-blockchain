pub struct Block {
  //ブロックの番号
  pub index: u64,
  //ブロックが生成された時間
  pub timestamp: u64,
  //配列でTransactionを保持
  pub transactions: Vec<Transaction>,

  pub proof: u64,
  //前のブロックのハッシュ値
  pub previous_hash: String,
}
