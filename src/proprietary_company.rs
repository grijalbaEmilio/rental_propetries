use crate::manager::Manager;

#[derive()]
pub struct ProprietaryCompany{
    pub nit:String,
    pub manager:Manager
}

impl ProprietaryCompany{

    pub fn new(nit: String, manager:Manager) -> ProprietaryCompany{
        ProprietaryCompany{
            nit,
            manager
        }
    }
}