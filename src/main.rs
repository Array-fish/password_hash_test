use password_hash::{Ident, ParamsString, PasswordHash, Salt};
fn main() {
    let db = DB{
        userid: "".to_string(),
        password: "".to_string(),
    };
    let userid = "rust";
    let password = "password";
    let login_ok = login(userid, password);
    println!("login_ok: {}", login_ok);
}

const EXAMPLE_ALGORITHM: Ident = Ident::new_unwrap("argon2d");
const EXAMPLE_SALT: &str = "somesalt";
const EXAMPLE_HASH: &[u8] = &[
    0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85,
    0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab,
];
/// Example parameters
fn example_params() -> ParamsString {
    let mut params = ParamsString::new();
    params.add_decimal("a", 1).unwrap();
    params.add_decimal("b", 2).unwrap();
    params.add_decimal("c", 3).unwrap();
    params
}
struct DB{
    pub userid: String,
    pub password: String,
}

impl DB {
    fn save(mut self, userid: &str, password: &str) {
        self.userid = userid.to_string();
        self.password = password.to_string();
    }
}
fn login(userid: &str, password: &str) -> bool {
    let password_hash = PasswordHash::try_from(password).unwrap();
    password_hash.verify_password(phfs, password)
    assert_eq!(
        s,
        "$argon2d$a=1,b=2,c=3$saltsaltsaltsaltsalt$hashhashhashhashhashhashhashhashhashhashhas"
    );

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    if userid == "rust" && password == "password" {
        true
    } else {
        false
    }
}