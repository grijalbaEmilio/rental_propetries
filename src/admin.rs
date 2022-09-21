pub struct Admin{
    pub identification: String,
}

impl Admin{
    pub fn new(identification:String) -> Admin{
        Admin{
            identification
        }
    }
}