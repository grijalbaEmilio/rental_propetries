pub struct PropertyType{
    pub code: String,
    pub description: String,
}

impl PropertyType{
    pub fn new(code: String, description: String) -> PropertyType{
        PropertyType{
            code,
            description
        }
    }
}