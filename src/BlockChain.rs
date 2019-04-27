pub struct BlockChain {
  //ユーザー識別子
  identifier: String,
  //ブロックを配列で管理
  blocks: Vec<Block>,
  //現在のトランザクションを管理(承認されていない)
  current_transactions: Vec<Transaction>,
}
impl BlockChain {
  //identify関数　引数にパスワードとパスフレーズを取る　戻り値は文字列
  pub fn identify(&mut self, password: &str, passphrase: &str) -> String {
    //identificationのgenerateにパスワードとパスフレーズを引数に取りBlockChainの構造体に格納
    self.identifier = identification::generate(&password, &passphrase);
    //identifierをクローンする
    self.identifier.clone()
  }

  //new_transaction関数　引数に送信者、受信者と取引量を取る　戻り値は64ビットの正の整数
  pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: u32) -> u64 {
    //BlockChainのcurrent_transactionに配列で保持しているTransactionを加える
    self.current_transactions.push(Transaction {
      //senderを文字列化したもの
      sender: sender.to_string(),
      //recipientを文字列化したもの
      recipient: recipient.to_string(),
      //取引量
      amount: amount,
    });
    //64ビットの正の整数でblocksの配列の長さを取得する
    self.blocks.len() as u64
  }

  //new_block関数　引数にタイムスタンプとproofを取る　戻り値は64ビットの正の整数
  fn new_block(&mut self, timestamp: u64, proof: u64) -> u64 {
    //current_indexに正の整数で取得したblocksの長さを格納する
    let current_index = self.blocks.len() as u64;
    //next_transactionにcurrent_transactionsを可変長配列にしたものを格納する
    let next_transactions = self.current_transactions.to_vec();
    //nextに右辺の結果を格納 blocksの最後の要素を取得し何もなかったら下のgenesisブロック、何かあったら上のブロックを格納
    let next = match self.blocks.last() {
      Some(previous) => {
        Block {
          index: current_index,
          timestamp: timestamp,
          proof: proof,
          previous_hash: previous.hash(),
          transactions: next_transactions
        }
      }
      None => {
        Block {
          index: 0,
          timestamp: timestamp,
          proof: proof,
          previous_hash: "genesis".to_string(),
          transactions: Vec::new(),
        }
      }
    };
    //BlockChainのblocksにnextを繋げる
    self.blocks.push(next);
    //current_transactionsに新しく作成した可変長配列を格納する
    self.current_transactions = Vec::new();

    //blocksの長さから1を引いたものを？
    (self.blocks.len() - 1) as u64
  }
}