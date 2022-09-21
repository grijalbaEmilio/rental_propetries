use crate::property_type::PropertyType;
use crate::admin::Admin;
use crate::proprietary_company::ProprietaryCompany;

pub struct Property{
    code: String,
    direction: String,
    area: f64,
    description: String,
    property_type: PropertyType,
    admin: Admin,
    proprietary_company: ProprietaryCompany
}

impl Property{
    pub fn new( code: String, direction: String, area: f64, description: String,
        property_type: PropertyType, admin: Admin, 
        proprietary_company: ProprietaryCompany) -> Property{
            Property{
                code,
                direction,
                area,
                description,
                property_type,
                admin,
                proprietary_company
            }
        }

    pub fn print_property(&self){
        println!("----- Property ----");
        println!("code: {}\ndirection: {}\narea: {}m²\ndescription: {}", &self.code, &self.direction, 
        &self.area, &self.description);
        println!("-------------------");
    }
}