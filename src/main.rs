use std::io;
use std::process;
use std::{thread, time};
extern crate rpassword;
use rpassword::read_password;

struct User {
    fName: String,
    lName: String,
    isActive: bool,
    password: String,
    team: String,
}

fn main() {
    banner();
    loginLoop();
    mainMenu();
    print!("\x1B[2J");
    println!("Goodbye from");
    banner();
}

fn loginLoop() {
    let mut looper = 3;
    loop {
        if looper == 0 {
            println!("Login Failed. Quitting.");
            process::abort();
        } else {
            let myUser = String::from(readUserInput(String::from("Please type username:")));
            //need a list of users here that can be matched to once a username is typed


            let myPassword = String::from(passwordInput(false));
            //need a list of passwords to be matched to

            if myUser.trim() == String::from("rhys"){
                match myPassword.as_str() {
                    "Password1" => {
                        println!("Success");
                        break;
                    }
                    _ => {
                        println!("Incorrect Password, try again.")
                    }
                }
            }
            looper += -1;
        }
    }
}

fn passwordInput(isCreating: bool) -> String {
    loop {
        println!("Please enter password: ");
        let password = String::from(rpassword::prompt_password("").unwrap());

        if isCreating == true {
            println!("Please enter password again: ");
            let secondPassword = String::from(rpassword::prompt_password("").unwrap());

            if password == secondPassword {
                println!("Passwords Match.");
                return password;
            } else {
                println!("Passwords do not match.");
            }
        } else {
            return password;
        }
    }
}

fn waitPlease() {
    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();
    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
}

fn banner() {
    //prints the banner
    println!("▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄");
    println!("██░███░█▄░▄██░▄▄▀██░▄▄▄░██░▀██░█████░▄▄▀██░▄▄▄██░███░");
    println!("██░█░█░██░███░▀▀▄██░███░██░█░█░█▀▀██░██░██░▄▄▄███░█░█");
    println!("██▄▀▄▀▄█▀░▀██░██░██░▀▀▀░██░██▄░█▄▄██░▀▀░██░▀▀▀███▄▀▄█");
    println!("▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀");
    println!("");
    println!("Wiron Project Managment v.001");
    println!("");
}

fn mainMenu() {
    //the main menu of our app. This is where the basic functions of the application will be accessed from
    let mut x: bool = true;

    while x == true {
        let mut userChoice = String::new();

        print!("\x1B[2J");
        banner();
        println!("Welcome to Wiron Project Management");
        println!("");

        //Add User Option
        printMenuOptions();
        println!("Please select and action from the below list: ");
        io::stdin()
            .read_line(&mut userChoice)
            .expect("Failed to read line.");
        userChoice = String::from(userChoice.trim());
        //match case for the menu please

        match userChoice.as_str() {
            "1" => createUser(),
            "2" => removeUser(),
            "3" => createTeam(),
            "4" => removeTeam(),
            "5" => moveTeam(),
            "6" => x = false,
            _ => println!("Invalid input, please try again."),
        }
    }
}

fn printMenuOptions() {
    println!("1. Add User.");
    println!("2. Remove User.");
    println!("3. Create Team.");
    println!("4. Remove Team.");
    println!("5. Change a users team.");
    println!("6. Exit");
    println!("");
}

fn readUserInput(x: String) -> String {
    //functions just for returning read lines
    println!("{}", x);
    let mut readLine = String::new();
    io::stdin()
        .read_line(&mut readLine)
        .expect("Failed to read line.");
    readLine
}

fn createUser() {
    //use User struct here
    let mut userInput = User {
        //initiating empty struct to be filled with user inputs.
        fName: String::from(readUserInput(String::from(
            "Please input users first name:",
        ))),
        lName: String::from(readUserInput(String::from("Please input users last name:"))),
        isActive: true,
        password: String::from(passwordInput(true)),
        team: String::from(readUserInput(String::from("Please input users team:"))),
    };
    println!();
    println!("Adding User......");
    waitPlease();
}

fn removeUser() {
    println!("Remove a user.");
}

fn createTeam() {
    println!("Create a team.");
}

fn removeTeam() {
    println!("Remove a team.");
}

fn moveTeam() {
    println!("Move a user to a different team.");
}
