fn cat(x: &str) -> &str {
  x
}


fn calculation(n: u64) -> u64 {
  if n == 0 {
    0
  } else {
    n + 1
  }
}

fn sample_loop(mut n: u64) -> u64 {
  let mut total: u64 = 0;
  loop {
    if n == 0 {
      break;
    }
    total += n;
    n -= 1;
    println!("===============");
    println!("{}", total);
    println!("{}", n);
  }
  total
}

fn try_diffence_return(n: i16) -> i16 {
  if n > 0 {
    n * n
  } else {
    n / n
  }
}

fn main() {
  let n = 1;
  let x: i16 = 3;
  let result = cat("ペルシャ猫が近所で泣いてるよ");
  // 変数をつかわない場合はprefix _numで表記する
  let _num: u64 = calculation(n);
  let hoge: i16 = try_diffence_return(x);
  sample_loop(10);
  println!("{}", result);
  println!("{}", _num);
  println!("{}", hoge);
}