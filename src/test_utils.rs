use serde_json::{from_str, to_string_pretty, Result};
use std::fmt::Debug;

fn check_result<T>(output: Result<T>, expected: Result<T>)
where
    T: Debug + PartialEq,
{
    match (output, expected) {
        (Ok(output), Ok(expected)) => {
            assert_eq!(output, expected);
        }
        (Ok(output), Err(why)) => {
            panic!("Expected {}, found Ok({:?})", why, output);
        }
        (Err(why), Ok(expected)) => {
            panic!("Expected {:?}, found Err({})", expected, why);
        }
        (Err(output), Err(expected)) => {
            assert_eq!(output.to_string(), expected.to_string());
        }
    }
}

pub fn check_ser<S: serde::Serialize>(input: &S, expected: Result<&str>) {
    let expected = expected.map(|s| s.to_owned());
    let output = to_string_pretty(input);

    check_result(output, expected);
}

pub fn check_de<'de, D>(input: &'de str, expected: Result<D>)
where
    D: Debug + PartialEq + serde::Deserialize<'de>,
{
    let output: Result<D> = from_str(input);

    check_result(output, expected);
}

pub fn check_serde<'de, T>(json: &'de str, object: T)
where
    T: Debug + PartialEq + serde::Deserialize<'de> + serde::Serialize,
{
    check_ser(&object, Ok(json));
    check_de(json, Ok(object));
}
