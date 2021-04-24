use std::{env::set_current_dir, error::Error, fmt::Display, fs::File, process::{self, Command}, thread::sleep, time::Duration};
use std::fmt::Debug;
use std::io::Write;
use std::io::Read;
use std::fs;
use anyhow::Context;

#[derive(Debug, Clone)]
struct TestError {
    msg: String,
    expected: String,
    actual: String,
}
impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test failed")
    }
}
impl Error for TestError {
}

fn assert_eq<T: PartialEq + Debug>(msg: impl ToString, expected: T, actual: T) -> anyhow::Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(TestError{
            msg: msg.to_string(),
            expected: format!("{:?}", expected), 
            actual: format!("{:?}", actual)
        }).context("assert_eq failed")
    }
}

fn dump_response(name: &str, response_text: &str) -> anyhow::Result<()> {
    let filename = format!("test-responses/response-{}.txt", name);
    fs::create_dir_all("test-responses")?;
    let mut file = File::create(&filename)?;
    file.write(response_text.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn test_response(name: &str, mime_type: &str, url: &str) -> anyhow::Result<()> {
    let response = reqwest::blocking::get(url)?;
    assert_eq(&format!("{}: Expect status code to be 200 (successful)", name), "200", response.status().as_str())?;
    let content_type_header = response.headers().get("content-type")
        .with_context(|| format!("{}: No content-type header found", name))?
        .to_str().with_context(|| format!("{}: Cannot convert content-type header to str", name))?;
    assert_eq(&format!("{}: Expect correct mime-type", name), mime_type, content_type_header)?;
    let response_text= response.text()?;
    dump_response(name, &response_text)?;
    
    let expected_filename = format!("expected-responses/response-{}.txt", name);
    let mut expected = String::new();
    File::open(&expected_filename)
        .with_context(|| format!("Could not open response file: {}", expected_filename))?
        .read_to_string(&mut expected)?;
    
    assert_eq(&format!("{}: Expect request body to match template", name), &expected, &response_text)?;

    Ok(())
}

fn cleanup(mut process: process::Child) {
    process.kill().expect("Couldn't kill server");
    set_current_dir("../..").expect("Couldn't change directory back");
}

#[test]
fn test() {
    set_current_dir("test-data/basic").expect("Couldn't change directory");
    let process = Command::new("cargo")
        .args(&["run"])
        .spawn()
        .expect("Couldn't start the server");
    match || -> anyhow::Result<()> {
        sleep(Duration::from_secs(2));

        test_response("index", "text/html", "http://localhost:2536")?;
        test_response("simple-post", "text/html", "http://localhost:2536/post/2019-11-26-simple-post.html")?;
        test_response("first-post", "text/html",  "http://localhost:2536/post/2019-11-24-first-post.html")?;
        test_response("static-text", "text/plain",  "http://localhost:2536/static/test.txt")?;
    
        Ok(())
    }() {
        Ok(_) => {
            cleanup(process);
        },
        Err(err) => {
            println!("{:?}", err);
            if let Some(test_error) = err.downcast_ref::<TestError>() {
                println!("Test MSG: {}", test_error.msg);
                println!("Expected: {}", test_error.expected);
                println!("Actual:   {}", test_error.actual);
            }
            cleanup(process);
            panic!();
        }
    };
}
