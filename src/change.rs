pub struct Change<'a> {
    pub location: String,
    pub changes: Vec<&'a str>
}