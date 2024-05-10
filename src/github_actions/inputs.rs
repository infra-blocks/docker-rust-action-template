pub type Input = Option<String>;

/// Returns the value of the input, as found in the environment.
///
/// This function parses the environment the same as core.getInput() does in their Javascript
/// SDK. However, core.getInput() returns an empty string when the environment variable isn't
/// defined. The GitHub Actions engine does the same when no value is provided for an input.
///
/// This function unifies the cases where the environment variable is not set or is empty.
/// It returns None if the input is not set in the environment, *or if the environment variable
/// is an empty string*.
pub fn get_input(name: &str) -> Input {
    let env_var_name = format!("INPUT_{}", name.to_uppercase().replace(' ', "_"));
    std::env::var(env_var_name)
        .ok()
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_word_input() {
        std::env::set_var("INPUT_SIMPLE", "big-value");
        assert_eq!(get_input("simple"), Some("big-value".to_string()));
        std::env::remove_var("INPUT_SIMPLE");
    }

    #[test]
    fn test_hyphenated_input() {
        std::env::set_var("INPUT_THIS-IS-LEGIT", "#believe");
        assert_eq!(get_input("this-is-legit"), Some("#believe".to_string()));
        std::env::remove_var("INPUT_THIS-IS-LEGIT");
    }

    #[test]
    fn test_camel_case_input() {
        std::env::set_var("INPUT_CAMELCASE", "miaw");
        assert_eq!(get_input("camelCase"), Some("miaw".to_string()));
        std::env::remove_var("INPUT_CAMELCASE");
    }

    #[test]
    fn test_missing_input() {
        assert_eq!(get_input("missing"), None);
    }

    #[test]
    fn test_empty_input() {
        std::env::set_var("INPUT_EMPTY", "");
        assert_eq!(get_input("empty"), None);
        std::env::remove_var("INPUT_EMPTY");
    }
}
