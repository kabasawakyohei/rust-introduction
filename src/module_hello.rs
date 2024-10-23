pub fn print_hello() {
  println!("Splitting modules");
}

// 『tips』
// pubをつけることでモジュールの要素へモジュール外からアクセスできるようになる
// つまりpubをつけずに以下の状態にすることでオブジェクト指向におけるクラスの変数やメソッドの隠蔽（カプセル化）のような状態になる
// fn print_hello(){
//   ....
// }
