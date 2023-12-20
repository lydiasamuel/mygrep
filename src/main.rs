use mygrep::postfixer;

fn main() {
    let tmp = postfixer::transform("(a|b)*a".to_string()).unwrap();

    for item in tmp {
        print!("{}", item.to_string());
    }
}