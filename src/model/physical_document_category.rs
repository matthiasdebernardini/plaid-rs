use serde::{Serialize, Deserialize};
/**The type of identity document detected in the images provided. Will always be one of the following values:

  `drivers_license` - A driver's license issued by the associated country, establishing identity without any guarantee as to citizenship, and granting driving privileges

  `id_card` - A general national identification card, distinct from driver's licenses as it only establishes identity

  `passport` - A travel passport issued by the associated country for one of its citizens

  `residence_permit_card` - An identity document issued by the associated country permitting a foreign citizen to <em>temporarily</em> reside there

  `resident_card` - An identity document issued by the associated country permitting a foreign citizen to <em>permanently</em> reside there

  `visa` - An identity document issued by the associated country permitting a foreign citizen entry for a short duration and for a specific purpose, typically no longer than 6 months

Note: This value may be different from the ID type that the user selects within Link. For example, if they select "Driver's License" but then submit a picture of a passport, this field will say `passport`*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PhysicalDocumentCategory {
    #[serde(rename = "drivers_license")]
    DriversLicense,
    #[serde(rename = "id_card")]
    IdCard,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "residence_permit_card")]
    ResidencePermitCard,
    #[serde(rename = "resident_card")]
    ResidentCard,
    #[serde(rename = "visa")]
    Visa,
}
