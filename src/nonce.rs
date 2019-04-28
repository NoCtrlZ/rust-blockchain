
impl Nonce {
  //次のNonceを見つける作業
  pub fn find_next(&self) -> u64 {
    //nonceに1をセット
    let mut nonce = 1;
    //verify関数にcurrent?とnonceを引数に取り偽だったら繰り返す
    while !Nonce::verify(self.current, nonce) {
      //nonceの値を+1する
      nonce += 1;
    }
    //nonceを戻り値として返す
    nonce
  }

  //verify関数に上で得たcurrent?とnonceを引数として取る
  pub fn verify(current: u64, next: u64) -> bool {
    //currentとnextを繋げたものをmessageに格納する
    let message = format!("{}{}", current, next);
    //messageに対してsha256を取ったものをdigestに格納する
    let digest = hash::sha256::digest(&message);
    //digestの最初が000で始まるかどうかで真偽値を取る
    digest.starts_with("000")
  }
}
