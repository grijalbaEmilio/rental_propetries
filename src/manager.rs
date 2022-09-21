pub struct Manager{
    pub identification: String,
}

impl Manager{
    pub fn new(identification: String) -> Manager{
        Manager{
            identification
        }
    }
}