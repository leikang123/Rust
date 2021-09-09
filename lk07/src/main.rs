fn main() {
    let s = String ::from("leikang");
    let s2 = s.clone();
    println!("s value is {:?}",s);
    let mut s3 = String :: from ("leikang");
    let  s4 = &mut s3;
       s4.push_str("oob");
    print!("s4 is {}",s4);
    let s5 = &mut s3;
    print!("s5 is {}",s5);
    let  s6 = String :: from ("hgfd");
    let r1 = &s6;
    let r2 = &s6;
    print!("r1={},r2={}",r1,r2);

}
