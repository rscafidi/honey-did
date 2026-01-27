use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegacyDocument {
    pub meta: DocumentMeta,
    pub financial: FinancialSection,
    pub insurance: InsuranceSection,
    pub bills: BillsSection,
    pub property: PropertySection,
    pub legal: LegalSection,
    pub digital: DigitalSection,
    pub household: HouseholdSection,
    pub personal: PersonalSection,
    pub contacts: ContactsSection,
    pub medical: MedicalSection,
    pub pets: PetsSection,
    pub welcome_screen: Option<WelcomeScreen>,
    #[serde(default)]
    pub custom_sections: Vec<CustomSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMeta {
    pub creator_name: String,
    pub created_at: String,
    pub updated_at: String,
}

// --- Financial Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialSection {
    pub bank_accounts: Vec<BankAccount>,
    pub credit_cards: Vec<CreditCard>,
    pub investments: Vec<Investment>,
    pub debts: Vec<Debt>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccount {
    pub name: String,
    pub institution: String,
    pub account_type: String,
    pub last_four: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditCard {
    pub name: String,
    pub issuer: String,
    pub last_four: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Investment {
    pub name: String,
    pub institution: String,
    pub account_type: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Debt {
    pub name: String,
    pub lender: String,
    pub notes: String,
}

// --- Insurance Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InsuranceSection {
    pub policies: Vec<InsurancePolicy>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InsurancePolicy {
    pub policy_type: String,  // life, health, home, auto, etc.
    pub provider: String,
    pub policy_number: String,
    pub contact: String,
    pub notes: String,
}

// --- Bills Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillsSection {
    pub bills: Vec<Bill>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bill {
    pub name: String,
    pub provider: String,
    pub amount: String,
    pub due_day: String,
    pub autopay: bool,
    pub notes: String,
}

// --- Property Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropertySection {
    pub properties: Vec<Property>,
    pub vehicles: Vec<Vehicle>,
    pub valuables: Vec<Valuable>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Property {
    pub name: String,
    pub address: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vehicle {
    pub name: String,
    pub details: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Valuable {
    pub name: String,
    pub location: String,
    pub notes: String,
}

// --- Legal Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalSection {
    pub will_location: String,
    pub attorney: Contact,
    pub power_of_attorney: String,
    pub trusts: Vec<Trust>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Trust {
    pub name: String,
    pub trustee: String,
    pub notes: String,
}

// --- Digital Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DigitalSection {
    pub email_accounts: Vec<DigitalAccount>,
    pub social_media: Vec<DigitalAccount>,
    pub password_manager: PasswordManagerInfo,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DigitalAccount {
    pub name: String,
    pub username: String,
    pub recovery_hint: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PasswordManagerInfo {
    pub name: String,
    pub master_password_hint: String,
    pub recovery_method: String,
    pub notes: String,
}

// --- Household Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HouseholdSection {
    pub maintenance_items: Vec<MaintenanceItem>,
    pub contractors: Vec<Contact>,
    pub how_things_work: Vec<HowTo>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MaintenanceItem {
    pub name: String,
    pub frequency: String,
    pub last_done: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HowTo {
    pub name: String,
    pub instructions: String,
}

// --- Personal Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalSection {
    pub funeral_preferences: String,
    pub obituary_notes: String,
    pub messages: Vec<PersonalMessage>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalMessage {
    pub recipient: String,
    pub message: String,
}

// --- Contacts Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContactsSection {
    pub emergency_contacts: Vec<Contact>,
    pub family: Vec<Contact>,
    pub professionals: Vec<Contact>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Contact {
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub email: String,
    pub notes: String,
}

// --- Medical Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MedicalSection {
    pub family_members: Vec<FamilyMedical>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FamilyMedical {
    pub name: String,
    pub doctors: Vec<Contact>,
    pub medications: Vec<Medication>,
    pub conditions: Vec<String>,
    pub allergies: Vec<String>,
    pub pharmacy: Contact,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Medication {
    pub name: String,
    pub dosage: String,
    pub frequency: String,
    pub prescriber: String,
    pub notes: String,
}

// --- Pets Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PetsSection {
    pub pets: Vec<Pet>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pet {
    pub name: String,
    pub species: String,
    pub breed: String,
    pub vet: Contact,
    pub medications: Vec<Medication>,
    pub feeding: String,
    pub care_notes: String,
}

// --- Welcome Screen Section ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum SlideType {
    #[default]
    Message,
    Question,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageSlide {
    pub id: String,
    #[serde(rename = "type")]
    pub slide_type: SlideType,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    pub transition: SlideTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlideTransition {
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "auto")]
    Auto { seconds: u32 },
}

impl Default for SlideTransition {
    fn default() -> Self {
        SlideTransition::Click
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WelcomeScreen {
    pub enabled: bool,
    pub slides: Vec<MessageSlide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_passphrase: Option<String>,
}

// --- Custom Sections ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomSection {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,  // None = top-level, Some("financial") = subsection
    pub subsections: Vec<CustomSubsection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomSubsection {
    pub id: String,
    pub name: String,
    pub field_definitions: Vec<FieldDefinition>,
    pub items: Vec<CustomItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldDefinition {
    pub id: String,
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    #[default]
    Text,
    Number,
    Date,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomItem {
    pub id: String,
    pub values: HashMap<String, String>,  // field_id -> value
}
