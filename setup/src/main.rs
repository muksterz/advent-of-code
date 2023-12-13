use keyring::Entry;

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap();


    match cmd.as_str() {
        "get" => get_token(),
        "set" => set_token(args.next().unwrap()),
        _ => panic!()
    }


}

fn get_token() {
    let entry = keyring::Entry::new("aoc_runner", &whoami::username()).unwrap();
    let token = entry.get_password().expect("No token found");
    println!("{token}");

}

fn set_token(token: String) {
    let user = whoami::username();
    let entry = Entry::new("aoc_runner", &user).unwrap();
    entry.set_password(&token).unwrap();
}