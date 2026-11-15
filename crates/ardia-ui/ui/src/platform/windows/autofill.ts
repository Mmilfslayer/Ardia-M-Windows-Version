import { invoke } from '@tauri-apps/api/core';

export interface CreditCard {
  guid: string;
  name: string;
  number: string;
  expiryMonth: string;
  expiryYear: string;
  nickname?: string;
}

export interface Address {
  guid: string;
  name: string;
  company?: string;
  address1: string;
  address2?: string;
  city: string;
  state: string;
  zipCode: string;
  country: string;
  phone?: string;
  email?: string;
}

export interface AutofillProfile {
  guid: string;
  name: string;
  email?: string;
  phone?: string;
  address?: Address;
  creditCards?: CreditCard[];
}

export async function getAutofillProfiles(): Promise<AutofillProfile[]> {
  return invoke('autofill_get_profiles');
}

export async function saveAutofillProfile(profile: Omit<AutofillProfile, 'guid'>): Promise<string> {
  return invoke('autofill_save_profile', { profile });
}

export async function deleteAutofillProfile(guid: string): Promise<void> {
  return invoke('autofill_delete_profile', { guid });
}

export async function fillForm(fieldType: 'address' | 'creditCard' | 'profile', data: any): Promise<void> {
  return invoke('autofill_fill_form', { fieldType, data });
}

export async function getAddressSuggestions(query: string): Promise<Address[]> {
  return invoke('autofill_get_address_suggestions', { query });
}

export async function getCreditCardSuggestions(query: string): Promise<CreditCard[]> {
  return invoke('autofill_get_credit_card_suggestions', { query });
}