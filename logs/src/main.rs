use std::fs;
use std::io::Error;

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        return Err(Error::other("Can't divide by 0"));
    }

    Ok(a / b)
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Emails must have an @"))
    }
}

fn string_test(a: String, b: &String, c: &str) {}

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];
    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    match fs::read_to_string("logs.txt") {
        Ok(text_on_file) => {
            let error_logs = extract_errors(text_on_file.as_str());
            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors .txt"),
                Err(reason) => {
                    println!("Failed on write: {}", reason)
                }
            }
        } // text_on_file will go out of scope
        Err(why_failed) => {
            println!("Error on read: {}", why_failed)
        }
    }

    let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    let error_logs = extract_errors(text.as_str());

    fs::write("errors.txt", error_logs.join("\n")).expect("Failed to write errors.txt");

    string_test(
        String::from("straw"),
        &String::from("dawn"),
        &String::from("nika"),
    );

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(mistake) => {
            println!("{}", mistake);
        }
    }

    match validate_email(String::from("luffysunny.com")) {
        Ok(..) => println!("Email is valid"),
        Err(reason) => {
            println!("{}", reason)
        }
    }

    Ok(())
}
