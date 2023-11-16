fn main() {
    //
    //所有権
    //

    //
    //String
    //
    /*
    //String型の生成
    let s = String::from("Hello");
    println!("{}",s);

    //Stringの可変化
    let mut s = String::from("hello");
    s.push_str(",world!"); //s+文字リテラル
    println!("{}",s);

    //有効性(ムーブ)
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();

    //println!("{}",s1); //パニック
    println!("s2 = {}, s3 = {}",s2,s3); //cloneしたものだと良い。
    */

    //
    //関数とスコープ
    //

    let s = String::from("hello"); //sスコープに入る
    takes_ownership(s);            //sが関数にムーブ
　//println!("{}",s);          //ERR,sがスコープを抜けている。

    let x = 5;
    makes_copy(x);

    //
    //戻り値とスコープ
    //

    let s1 = gives_ownership();   //gives_ownership => s1　でムーブ

    let s2 = String::from("hello");  // s2 がスコープに入る。

    let s3 = takes_and_gives_backs(s2); //s2 => takes_and_gives_backs　でムーブ、さらに　takes_and_gives_backs => s3　でムーブ

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}",some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_backs(a_string: String) -> String {
    a_string
}
