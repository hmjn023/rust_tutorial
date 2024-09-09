mod func;
//use func::hello::hoge;
//use func::kazuate::kazuate;
//use func::loops::test_for;
use func::structs::structs;

fn main() {
    // func::hello::hello();
    // println!("{}", hoge())
    // kazuate();
    // test_for();
    //let hoge: func::structs::User = structs();
    //hoge.userme = String::from("hgoe");
    println!(
        "{:?}",
        func::structs::User::dup(String::from("aa"), String::from("aa"), 12, false)
    )
}
