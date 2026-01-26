export interface WizardStepContent {
  id: string;
  title: string;
  icon: string;
  whyItMatters: string;
  prompts: string[];
  example: string;
}

export const prioritySteps: WizardStepContent[] = [
  {
    id: 'financial',
    title: 'Financial',
    icon: '💰',
    whyItMatters: 'Bank accounts, investments, and debts are essential for your family to manage finances, pay bills, and access funds during a difficult time.',
    prompts: [
      'Do you have any accounts your partner doesn\'t know about?',
      'Is there a safety deposit box? Where is the key?',
      'Any cryptocurrency or digital assets?',
      'Are there any automatic payments set up?',
    ],
    example: 'Chase Bank - Joint Checking - Last 4: 1234 - Primary account for bills',
  },
  {
    id: 'insurance',
    title: 'Insurance',
    icon: '🛡️',
    whyItMatters: 'Insurance policies provide financial protection. Without this information, claims could be delayed or missed entirely.',
    prompts: [
      'Do you have life insurance through your employer?',
      'Are there any policies with cash value?',
      'Who are the beneficiaries on each policy?',
      'Where are the policy documents stored?',
    ],
    example: 'Term Life - Northwestern Mutual - Policy #12345 - $500k - Spouse is beneficiary',
  },
  {
    id: 'legal',
    title: 'Legal',
    icon: '⚖️',
    whyItMatters: 'Legal documents like wills and powers of attorney ensure your wishes are followed and someone can act on your behalf if needed.',
    prompts: [
      'Where are the original documents stored?',
      'Who is your estate attorney?',
      'Is there a trust? Who is the trustee?',
      'Have you designated powers of attorney?',
    ],
    example: 'Will stored in home safe - Combination: ask John Smith (attorney) - Last updated 2024',
  },
  {
    id: 'medical',
    title: 'Medical',
    icon: '🏥',
    whyItMatters: 'Medical information helps family make informed decisions and ensures continuity of care during emergencies.',
    prompts: [
      'Any allergies or drug interactions to know about?',
      'Do you have an advance directive or living will?',
      'Who is your primary care doctor?',
      'Are there any ongoing treatments or medications?',
    ],
    example: 'Dr. Sarah Johnson - Primary Care - (555) 123-4567 - Annual checkup in March',
  },
  {
    id: 'contacts',
    title: 'Contacts',
    icon: '📞',
    whyItMatters: 'Knowing who to call first saves precious time and ensures the right people are notified.',
    prompts: [
      'Who should be called first in an emergency?',
      'Is there anyone who should NOT be contacted?',
      'Who has spare keys to your home?',
      'Any important work contacts to notify?',
    ],
    example: 'John Smith (brother) - (555) 987-6543 - Call first, has spare key',
  },
];

export const secondarySteps: WizardStepContent[] = [
  {
    id: 'bills',
    title: 'Bills',
    icon: '📄',
    whyItMatters: 'Regular bills need to be paid to avoid service interruptions and late fees.',
    prompts: [
      'Which bills are on autopay?',
      'Are there any annual payments that might be forgotten?',
      'What accounts are bills paid from?',
    ],
    example: 'Electric - ConEd - $150/month - Autopay from Chase checking',
  },
  {
    id: 'property',
    title: 'Property',
    icon: '🏠',
    whyItMatters: 'Property details help manage real estate, vehicles, and valuable items.',
    prompts: [
      'Where are property deeds and titles stored?',
      'Any rental properties or timeshares?',
      'Valuable items that should be appraised?',
    ],
    example: 'Home - 123 Main St - Deed in safe deposit box - Mortgage with Wells Fargo',
  },
  {
    id: 'digital',
    title: 'Digital Life',
    icon: '💻',
    whyItMatters: 'Digital accounts contain important information and memories that shouldn\'t be lost.',
    prompts: [
      'Do you use a password manager?',
      'Any subscriptions that should be cancelled?',
      'Where are photos and important files backed up?',
    ],
    example: '1Password - Master password in fireproof safe - Family vault shared',
  },
  {
    id: 'household',
    title: 'Household',
    icon: '🔧',
    whyItMatters: 'Household knowledge keeps the home running smoothly.',
    prompts: [
      'Where is the water shutoff valve?',
      'Any regular maintenance schedules?',
      'Trusted contractors for repairs?',
    ],
    example: 'Water shutoff - basement, left of stairs - red handle',
  },
  {
    id: 'personal',
    title: 'Personal',
    icon: '💝',
    whyItMatters: 'Personal wishes and messages provide comfort and guidance to loved ones.',
    prompts: [
      'Any funeral or memorial preferences?',
      'Messages you\'d want loved ones to have?',
      'Special items to pass to specific people?',
    ],
    example: 'Prefer cremation - scatter ashes at Lake Tahoe',
  },
  {
    id: 'pets',
    title: 'Pets',
    icon: '🐾',
    whyItMatters: 'Pets need continued care and someone who knows their routines.',
    prompts: [
      'Who should care for your pets?',
      'Any medical conditions or special diets?',
      'Where is the vet\'s information?',
    ],
    example: 'Max (golden retriever) - Takes thyroid medication daily - Vet: Dr. Paws (555) 222-3333',
  },
];
