pub struct ImportantExcerpt<'a> {
    pub part: &'a str
}
// This means the reference to ImportantExcerpt can't outlive the reference of part field
