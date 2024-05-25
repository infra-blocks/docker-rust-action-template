use std::fs::File;
use std::io::Write;
use std::path::Path;

use uuid::Uuid;

pub struct Output<'a> {
    key: &'a str,
    value: String,
}

impl<'a, T: ToString> From<(&'a str, T)> for Output<'a> {
    fn from((key, value): (&'a str, T)) -> Self {
        Self {
            key,
            value: value.to_string(),
        }
    }
}

impl<'a> Output<'a> {
    fn serialize_for_file_output(&self) -> String {
        let delimiter = format!("ghadelimiter_{}", Uuid::new_v4());

        if self.key.contains(&delimiter) {
            panic!(
                "Unexpected input: name should not contain the delimiter {}",
                delimiter
            );
        }

        if self.value.contains(&delimiter) {
            panic!(
                "Unexpected input: value should not contain the delimiter {}",
                delimiter
            );
        }

        format!("{}<<{}\n{}\n{}", self.key, delimiter, self.value, delimiter)
    }

    fn write_to_file(&self, file_path: &Path) -> std::io::Result<()> {
        if !file_path.exists() {
            panic!("missing file at path: {:?}", file_path);
        }

        // Open the file at the given path and append the formatted message to it.
        let message = self.serialize_for_file_output();
        let mut file = File::options().append(true).open(file_path)?;
        writeln!(&mut file, "{}", message)?;
        Ok(())
    }

    fn execute_as_command(&self) {
        println!();
        println!(
            "::set-output name={}::{}",
            escape_command_property(self.key),
            self.value
        );
    }
}

fn escape_command_property(property: &str) -> String {
    property
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}

pub fn set_output<'a, T: Into<Output<'a>>>(output: T) {
    let file_path = std::env::var("GITHUB_OUTPUT")
        .ok()
        .filter(|value| !value.is_empty());
    match file_path {
        Some(file_path) => {
            output.into().write_to_file(Path::new(&file_path)).unwrap();
        }
        None => {
            output.into().execute_as_command();
        }
    }
}

pub fn set_outputs<'a, I, T>(outputs: I)
where
    I: IntoIterator<Item = T>,
    T: Into<Output<'a>>,
{
    // TODO: use buffered file writer for performance. Would need tests fuuuuurscht.
    for output in outputs {
        set_output(output);
    }
}
