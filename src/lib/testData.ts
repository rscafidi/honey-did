import type { LegacyDocument } from './stores/document';

/**
 * Generates a fully populated LegacyDocument with realistic test data
 * for every section and field. Used for testing exports in dev mode.
 */
export function createTestDocument(): LegacyDocument {
  return {
    meta: {
      creator_name: 'Test User',
      created_at: '2024-01-15T10:00:00.000Z',
      updated_at: new Date().toISOString(),
    },
    financial: {
      bank_accounts: [
        { name: 'Primary Checking', institution: 'Chase Bank', account_type: 'Checking', last_four: '4821', notes: 'Direct deposit goes here' },
        { name: 'Savings', institution: 'Ally Bank', account_type: 'High-Yield Savings', last_four: '7733', notes: 'Emergency fund - 6 months expenses' },
      ],
      credit_cards: [
        { name: 'Cashback Visa', issuer: 'Citi', last_four: '9012', notes: '2% cashback on everything, autopay enabled' },
        { name: 'Travel Rewards', issuer: 'American Express', last_four: '3456', notes: 'Annual fee $95, keep open for points' },
      ],
      investments: [
        { name: 'Employer 401k', institution: 'Fidelity', account_type: '401k', notes: 'Employer matches 4%, vested after 3 years' },
        { name: 'Roth IRA', institution: 'Vanguard', account_type: 'Roth IRA', notes: 'Target date fund 2055' },
        { name: 'Brokerage', institution: 'Schwab', account_type: 'Taxable Brokerage', notes: 'Index funds, long-term hold' },
      ],
      debts: [
        { name: 'Home Mortgage', lender: 'Wells Fargo', notes: '30-year fixed at 3.25%, $285,000 remaining' },
        { name: 'Auto Loan', lender: 'Capital One', notes: '$12,400 remaining, payoff date March 2026' },
      ],
      notes: 'All accounts are joint with spouse. Financial advisor is Mark Thompson at Edward Jones (555-0150).',
    },
    insurance: {
      policies: [
        { policy_type: 'Life Insurance', provider: 'Northwestern Mutual', policy_number: 'NM-2024-88431', contact: '800-388-8123', notes: '$500,000 term life, beneficiary is spouse' },
        { policy_type: 'Homeowners', provider: 'State Farm', policy_number: 'SF-HO-445921', contact: '800-732-5246', notes: 'Agent: Lisa Park, annual premium $1,800' },
        { policy_type: 'Auto Insurance', provider: 'GEICO', policy_number: 'GK-9912-A', contact: '800-841-3000', notes: 'Covers both vehicles, $500 deductible' },
        { policy_type: 'Umbrella', provider: 'State Farm', policy_number: 'SF-UB-112003', contact: '800-732-5246', notes: '$1M umbrella policy, same agent as homeowners' },
      ],
      notes: 'Review all policies each November during open enrollment.',
    },
    bills: {
      bills: [
        { name: 'Mortgage', provider: 'Wells Fargo', amount: '$1,842', due_day: '1', autopay: true, notes: 'Includes escrow for taxes and insurance' },
        { name: 'Electric', provider: 'Duke Energy', amount: '$120-180', due_day: '15', autopay: true, notes: 'Budget billing, adjusts in April' },
        { name: 'Water & Sewer', provider: 'City Utilities', amount: '$65', due_day: '20', autopay: false, notes: 'Pay online at cityutil.gov, account #44821' },
        { name: 'Internet', provider: 'AT&T Fiber', amount: '$79.99', due_day: '8', autopay: true, notes: '1 Gbps plan, router is in office closet' },
        { name: 'Cell Phones', provider: 'T-Mobile', amount: '$140', due_day: '12', autopay: true, notes: 'Family plan, 4 lines' },
        { name: 'Streaming Bundle', provider: 'Various', amount: '$45', due_day: '5', autopay: true, notes: 'Netflix $15, Spotify $11, Disney+ $8, Hulu $8, iCloud $3' },
      ],
      notes: 'All autopay bills are linked to the Chase checking account.',
    },
    property: {
      properties: [
        { name: 'Family Home', address: '742 Evergreen Terrace, Springfield, IL 62704', notes: 'Purchased 2018, 3BR/2BA, 1,850 sqft. HOA is $150/quarter.' },
      ],
      vehicles: [
        { name: '2022 Honda CR-V', details: 'VIN: 2HKRW2H53NH012345, License: ABC 1234', notes: 'Oil change every 7,500 miles, dealership is Honda of Springfield' },
        { name: '2020 Toyota Camry', details: 'VIN: 4T1B11HK5LU012345, License: XYZ 5678', notes: 'Paid off, title is in the fireproof safe' },
      ],
      valuables: [
        { name: 'Engagement Ring', location: 'Safe deposit box #218 at Chase, downtown branch', notes: 'Appraised at $8,500 in 2023, insurance rider on homeowners policy' },
        { name: 'Coin Collection', location: 'Fireproof safe in basement office', notes: 'Inherited from grandfather, appraised at $3,200' },
        { name: 'Guitar Collection', location: 'Music room', notes: '1965 Fender Stratocaster ($12k), 2019 Gibson Les Paul ($2.5k), 1998 Martin D-28 ($3k)' },
      ],
      notes: 'Safe deposit box key is in the desk drawer in the home office. Spare house keys are with the Johnsons next door.',
    },
    legal: {
      will_location: 'Original is in the fireproof safe in the basement office. Copy with attorney Robert Chen.',
      attorney: { name: 'Robert Chen, Esq.', relationship: 'Estate Planning Attorney', phone: '555-0172', email: 'rchen@chenlaw.com', notes: 'At Chen & Associates, 400 Main St Suite 300. Last will update was January 2024.' },
      power_of_attorney: 'Spouse is primary POA (financial and healthcare). Sister Mary Johnson is backup.',
      trusts: [
        { name: 'Johnson Family Revocable Trust', trustee: 'Spouse (primary), Mary Johnson (successor)', notes: 'Holds the house and investment accounts. Annual review with attorney.' },
      ],
      notes: 'All estate documents were last reviewed January 2024. Next review scheduled for January 2025.',
    },
    digital: {
      email_accounts: [
        { name: 'Personal Gmail', username: 'testuser@gmail.com', recovery_hint: 'Recovery phone is the cell number, backup email is the work email', notes: 'Primary email for all personal accounts' },
        { name: 'Work Email', username: 'tjohnson@acmecorp.com', recovery_hint: 'IT department can reset: helpdesk@acmecorp.com', notes: 'Notify HR if anything happens' },
      ],
      social_media: [
        { name: 'Facebook', username: 'Tom Johnson', recovery_hint: 'Uses personal Gmail for login', notes: 'Has legacy contact set to spouse' },
        { name: 'LinkedIn', username: 'tom-johnson-springfield', recovery_hint: 'Personal Gmail', notes: 'Professional network, notify colleagues' },
        { name: 'Instagram', username: '@tomj_photos', recovery_hint: 'Personal Gmail', notes: 'Photography hobby account' },
      ],
      password_manager: { name: '1Password', master_password_hint: 'The name of our first apartment complex + the year we moved in + !', recovery_method: 'Emergency Kit PDF is printed and stored in the fireproof safe', notes: 'Family plan, spouse also has access. All other passwords are stored here.' },
      notes: 'All passwords are in 1Password. The master password hint above should be enough for spouse to figure it out.',
    },
    household: {
      maintenance_items: [
        { name: 'HVAC Filter', frequency: 'Every 3 months', notes: 'Use MERV 13, size 20x25x1. Filters are in the garage on the shelf.' },
        { name: 'Gutter Cleaning', frequency: 'Twice yearly (spring & fall)', notes: 'Can DIY with the extension ladder or call CleanPro ($150)' },
        { name: 'Water Heater Flush', frequency: 'Annually', notes: 'Turn off power first, attach hose to drain valve, run for 5 min' },
        { name: 'Smoke Detector Batteries', frequency: 'Every 6 months', notes: 'All 6 detectors use 9V batteries. Spare batteries in the junk drawer.' },
      ],
      contractors: [
        { name: 'Mike\'s Plumbing', relationship: 'Plumber', phone: '555-0188', email: 'mike@mikesplumbing.com', notes: 'Very reliable, fair prices. Ask for Mike directly.' },
        { name: 'Green Lawn Care', relationship: 'Lawn Service', phone: '555-0199', email: '', notes: 'Comes every Thursday April-October, $45/visit' },
        { name: 'Dave\'s Electric', relationship: 'Electrician', phone: '555-0211', email: 'dave@sparkelectric.com', notes: 'Licensed and insured, did the panel upgrade in 2023' },
      ],
      how_things_work: [
        { name: 'Sprinkler System', instructions: 'Controller is in the garage by the door. Zones 1-4 are front yard, 5-7 are backyard. Winterize by November 1st - shut off main valve in basement, then run each zone for 30 seconds to drain.' },
        { name: 'Sump Pump', instructions: 'In the basement utility room. Has battery backup. Test monthly by pouring a bucket of water in the pit. If the float switch sticks, jiggle it. Backup battery replaced every 3 years (last: 2023).' },
        { name: 'Smart Home', instructions: 'Everything runs through the SmartThings hub on the shelf in the office closet. App login is in 1Password. Covers: front door lock, garage door, thermostat, and security cameras.' },
      ],
      notes: 'Tool collection is in the garage. Snow blower is a Toro, manual is in the filing cabinet.',
    },
    personal: {
      funeral_preferences: 'I would prefer cremation. No formal funeral service - instead, have a casual celebration of life gathering at the house or a park. Play my favorite playlist (saved in Spotify as "The Good Stuff"). Scatter ashes at Lake Michigan near the spot where we got engaged.',
      obituary_notes: 'Mention my 15 years of volunteer work at the Springfield Food Bank, my love of photography and music, and my role as a proud parent and spouse. Keep it warm and personal, not formal.',
      messages: [
        { recipient: 'Spouse', message: 'You are the best thing that ever happened to me. Thank you for every single day. I love you more than words can say. Take care of yourself and don\'t be afraid to lean on our friends and family. You are stronger than you know.' },
        { recipient: 'Kids', message: 'I am so incredibly proud of the people you are becoming. Be kind, work hard, and never stop being curious. Take care of each other and your mom/dad. I\'ll always be with you in spirit. Love, Dad/Mom.' },
        { recipient: 'Best Friend (Dave)', message: 'Thanks for 20 years of friendship, terrible jokes, and great adventures. Look after my family for me. And yes, you can have the fishing gear.' },
      ],
      notes: 'Organ donor - registered. Spotify playlist "The Good Stuff" has my favorite songs for the celebration of life.',
    },
    contacts: {
      emergency_contacts: [
        { name: 'Sarah Johnson', relationship: 'Spouse', phone: '555-0101', email: 'sarah.j@gmail.com', notes: 'First call for anything' },
        { name: 'Mary Johnson', relationship: 'Sister', phone: '555-0102', email: 'mary.johnson@email.com', notes: 'Backup POA, lives 30 minutes away' },
      ],
      family: [
        { name: 'Robert Johnson Sr.', relationship: 'Father', phone: '555-0103', email: '', notes: 'Lives in Florida, health is good' },
        { name: 'Linda Johnson', relationship: 'Mother', phone: '555-0104', email: 'linda.j@email.com', notes: 'Same address as Dad' },
        { name: 'Dave & Jen Miller', relationship: 'Neighbors & Close Friends', phone: '555-0105', email: 'davemiller@email.com', notes: 'Have spare house key, know the alarm code' },
      ],
      professionals: [
        { name: 'Dr. Amanda Foster', relationship: 'Primary Care Physician', phone: '555-0106', email: '', notes: 'Springfield Medical Group, sees whole family' },
        { name: 'Mark Thompson', relationship: 'Financial Advisor', phone: '555-0150', email: 'mthompson@edwardjones.com', notes: 'Edward Jones, manages 401k rollover and IRA' },
        { name: 'Lisa Park', relationship: 'Insurance Agent', phone: '555-0151', email: 'lisa.park@statefarm.com', notes: 'State Farm, handles home and umbrella policies' },
        { name: 'Sandra Wu, CPA', relationship: 'Tax Accountant', phone: '555-0152', email: 'swu@wutax.com', notes: 'Files our taxes every year, has copies of last 7 years' },
      ],
      notes: 'The Millers next door have a spare key and know the alarm code. In an emergency, call spouse first, then sister Mary.',
    },
    medical: {
      family_members: [
        {
          name: 'Self (Tom Johnson)',
          doctors: [
            { name: 'Dr. Amanda Foster', relationship: 'Primary Care', phone: '555-0106', email: '', notes: 'Annual physical in March' },
            { name: 'Dr. James Lee', relationship: 'Cardiologist', phone: '555-0160', email: '', notes: 'See annually for high blood pressure monitoring' },
          ],
          medications: [
            { name: 'Lisinopril', dosage: '10mg', frequency: 'Once daily, morning', prescriber: 'Dr. Lee', notes: 'For blood pressure, been on since 2022' },
            { name: 'Vitamin D3', dosage: '2000 IU', frequency: 'Once daily', prescriber: 'Dr. Foster', notes: 'Over the counter, take with food' },
          ],
          conditions: ['Hypertension (controlled)', 'Seasonal allergies'],
          allergies: ['Sulfa drugs', 'Shellfish (mild)'],
          pharmacy: { name: 'CVS Pharmacy', relationship: 'Pharmacy', phone: '555-0170', email: '', notes: 'Corner of Main & Oak, auto-refill is set up' },
          notes: 'Blood type is O+. Health records available through MyChart app (login in 1Password).',
        },
        {
          name: 'Spouse (Sarah Johnson)',
          doctors: [
            { name: 'Dr. Amanda Foster', relationship: 'Primary Care', phone: '555-0106', email: '', notes: 'Same PCP as me' },
            { name: 'Dr. Rachel Kim', relationship: 'OB-GYN', phone: '555-0161', email: '', notes: 'Springfield Women\'s Health' },
          ],
          medications: [
            { name: 'Synthroid', dosage: '50mcg', frequency: 'Once daily, morning on empty stomach', prescriber: 'Dr. Foster', notes: 'For hypothyroidism, take 30 min before eating' },
          ],
          conditions: ['Hypothyroidism'],
          allergies: ['Penicillin'],
          pharmacy: { name: 'CVS Pharmacy', relationship: 'Pharmacy', phone: '555-0170', email: '', notes: 'Same CVS as mine' },
          notes: 'Blood type is A+.',
        },
      ],
      notes: 'Both of us use MyChart for medical records. Health insurance cards are in the filing cabinet under "Insurance."',
    },
    pets: {
      pets: [
        {
          name: 'Buddy',
          species: 'Dog',
          breed: 'Golden Retriever',
          vet: { name: 'Dr. Sarah Patel', relationship: 'Veterinarian', phone: '555-0180', email: 'info@springfieldvet.com', notes: 'Springfield Veterinary Clinic, 200 Oak Street' },
          medications: [
            { name: 'Heartgard', dosage: '1 chewable', frequency: 'Monthly (1st of each month)', prescriber: 'Dr. Patel', notes: 'Heartworm prevention, give with food' },
            { name: 'NexGard', dosage: '1 chewable', frequency: 'Monthly (1st of each month)', prescriber: 'Dr. Patel', notes: 'Flea and tick prevention' },
          ],
          feeding: '2 cups of Blue Buffalo Adult Large Breed, twice daily (morning and 5pm). Fresh water always available. No table scraps - he has a sensitive stomach.',
          care_notes: 'Walks: 30 min morning, 30 min evening. Loves the dog park on weekends. Afraid of thunderstorms - give calming treat from the pantry. Groomed every 8 weeks at PetSmart. Microchip #: 985112345678900.',
        },
        {
          name: 'Whiskers',
          species: 'Cat',
          breed: 'Domestic Shorthair (tabby)',
          vet: { name: 'Dr. Sarah Patel', relationship: 'Veterinarian', phone: '555-0180', email: 'info@springfieldvet.com', notes: 'Same vet as Buddy' },
          medications: [],
          feeding: '1/3 cup dry food (Royal Canin Indoor) morning and evening. One can of wet food as a treat on weekends. Fresh water, cleaned daily.',
          care_notes: 'Indoor only. Litter box in the laundry room, scoop daily, full change weekly. Likes to hide in the bedroom closet. Annual checkup in September. Microchip #: 985112345679001.',
        },
      ],
      notes: 'If we\'re both unavailable, the Millers next door can take care of them short-term. Long-term, sister Mary has agreed to take Buddy, and friend Jen wants Whiskers.',
    },
    welcome_screen: {
      enabled: true,
      slides: [
        { id: 'ws1', type: 'message', text: 'If you\'re reading this, it means I\'m no longer able to be there in person. I want you to know that planning for this was an act of love - I wanted to make things as easy as possible for you during a difficult time.', transition: { type: 'click' } },
        { id: 'ws2', type: 'message', text: 'This document contains everything you should need: financial accounts, insurance policies, contacts, passwords, and personal messages. Take your time going through it.', transition: { type: 'click' } },
        { id: 'ws3', type: 'question', text: 'What was the name of the restaurant where we had our first date?', answer: 'olive garden', transition: { type: 'click' } },
        { id: 'ws4', type: 'question', text: 'What is the name of our first pet together?', answer: 'buddy', transition: { type: 'click' } },
      ],
      fallback_passphrase: 'testing-export-fallback-2024',
    },
    custom_sections: [
      {
        id: 'custom1',
        name: 'Home Improvement Projects',
        subsections: [
          {
            id: 'sub1',
            name: 'Planned Projects',
            form_elements: [
              { type: 'header' as const, id: 'h1', text: 'Project Details' },
              { type: 'field' as const, id: 'proj_name', name: 'Project', field_type: 'text' as const },
              { type: 'field' as const, id: 'proj_budget', name: 'Budget', field_type: 'number' as const },
              { type: 'field' as const, id: 'proj_deadline', name: 'Target Date', field_type: 'date' as const },
              { type: 'divider' as const, id: 'd1' },
              { type: 'field' as const, id: 'proj_started', name: 'Started', field_type: 'boolean' as const },
            ],
            items: [
              { id: 'item1', values: { proj_name: 'Kitchen backsplash', proj_budget: '800', proj_deadline: '2024-06-01', proj_started: 'false', _notes: 'Subway tile, white. Materials at Home Depot.' } },
              { id: 'item2', values: { proj_name: 'Deck refinishing', proj_budget: '400', proj_deadline: '2024-05-15', proj_started: 'true', _notes: 'Already sanded. Need to apply Thompson\'s WaterSeal.' } },
            ],
          },
        ],
      },
    ],
  };
}
