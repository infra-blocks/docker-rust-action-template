type HandlerError = String;

pub struct HandlerParams {
    pub example_input: String,
}

pub fn handler<'a, I>(
    inputs: I,
) -> Result<impl IntoIterator<Item = (&'a str, String)>, HandlerError>
where
    I: Into<HandlerParams>,
{
    let params = inputs.into();
    Ok(vec![(
        "example-output",
        format!("this is your input: {}!", params.example_input),
    )])
}
