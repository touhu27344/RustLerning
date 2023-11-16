fn main() {
    //
    //所有権
    //

    //
    //String
    //

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



}
