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

    /*
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
        */

    //
    //参照と借用
    //
    /*
    //&s:参照 &mut s:可変参照

    let s1 = String::from("hello");
    let len = caluculate_length(&s1);

    println!("The length of '{}' is {}.",s1,len);

    //可変な参照
    let mut s = String::from("hello");

    change(&mut s);

    */

    //
    //スライス型
    //

    let mut s = String::from("hello world");

    let word = first_word(&s);
    let hello = &s[0..5];
    let world = &s[6..11];

    s.clear();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_backs(a_string: String) -> String {
    a_string
}

fn caluculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
