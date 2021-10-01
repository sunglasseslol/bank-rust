mod user;

//Main function that will import all files, will give status updates on the bank etc.
fn main() {
    println!("VirtualBank is up and running");
    user::create_account();
}
