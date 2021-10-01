use std::io::stdin;

pub fn create_account() {
    let user: (i32, &str, &str);
    let mut inp_name = String::new();
    let mut inp_birth = String::new();
    let mut inp_gen = String::new();
    let mut inp_ccn = String::new();
    let mut inp_dcn = String::new();
    let mut inp_occ = String::new();
    let mut dc_or_cc = String::new();
    let mut inp_y_n = String::new();
    let mut s_i = String::new();
    let mut ps = "";
    let details: bool;
    println!("Enter your name: ");
    stdin().read_line(&mut inp_name)
        .ok()
        .expect("Failed to read line");
    print!("Name: {}", inp_name);
    println!("Enter your DOB ( Syntax: ddmmyyyy )");
    stdin().read_line(&mut inp_birth)
        .ok()
        .expect("Failed to read line");
    print!("DOB: {}", inp_birth);
    println!("Enter your Gender ( Values: Male, Female )");
    stdin().read_line(&mut inp_gen)
        .ok()
        .expect("Failed to read line");
    print!("Gender: {}", inp_gen);
    println!("Do you have a Credit Card or a Debit Card? ( Values: Credit Card, Debit Card )");
    stdin().read_line(&mut dc_or_cc)
        .ok()
        .expect("Failed to read line");
    println!("{}", dc_or_cc);
    if dc_or_cc == "Credit" {
        stdin().read_line(&mut inp_ccn)
            .ok()
            .expect("Failed to read line");
        ps = "Credit Card";
    }
    else if dc_or_cc == "Debit" {
        stdin().read_line(&mut inp_dcn)
            .ok()
            .expect("Failed to read line");
        ps = "Debit Card";
    }
    print!("Gender: {}", inp_gen);
    println!("-----------------");
    println!("Your name is {}, You were born on {}, Your a {}, and you have a {}, Correct? ( Values: y = yes, n = no )", inp_name, inp_birth, inp_gen, dc_or_cc);
    stdin().read_line(&mut inp_y_n)
            .ok()
            .expect("Failed to read line");
    if inp_y_n == "y" {
        details = true;
        println!("Your account has been made, would you like to sign in? ( Values: y, n");
        stdin().read_line(&mut s_i)
            .ok()
            .expect("Failed to read line");
        if s_i == "y" {
            println!("You have been signed in!");
        }
        else if s_i == "n" {
            println!("You have NOT been signed in");
        }
    }
    else if inp_y_n == "n" {
        details = false;
        println!("We're sorry for the inconvenience, please restart the program and try again.");
    }

}
pub fn login_account() {

}