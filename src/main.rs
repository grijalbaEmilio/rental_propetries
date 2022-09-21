mod property_type;
mod admin;
mod manager;
mod proprietary_company;
mod property;

use property_type::PropertyType;
use admin::Admin;
use manager::Manager;
use proprietary_company::ProprietaryCompany;
use property::Property;

fn main() {
    let property_type_one = PropertyType::new("0001".to_string(), "finca".to_string());
    println!("the one property type is: {}", property_type_one.description);

    let admin_one = Admin::new(format!("108800021"));
    println!("the identification from admin is: {}", admin_one.identification);

    let manager_one = Manager::new(format!("100031249"));
    println!("the identification from manager is: {}", manager_one.identification);

    let company_one= ProprietaryCompany::new("0012".to_string(), manager_one);
    println!("the nit company is: {}", company_one.nit);

    let property_one = Property::new(format!("001"), format!("call-13 #23-40"),
    34.12, format!("es una fínca clásica de seiglo XIX"), property_type_one, admin_one,
    company_one);

    property_one.print_property();


}
