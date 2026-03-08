import { writable, derived } from 'svelte/store';

export interface WizardState {
  step: number;
  identity: { name: string; url: string; email: string; photo: string };
  site: { name: string; directory: string };
  templateLang: string;
  css: string;
  indieweb: { webmention: boolean; micropub: boolean; indieauth: boolean };
}

const defaultState: WizardState = {
  step: 0,
  identity: { name: '', url: '', email: '', photo: '' },
  site: { name: '', directory: '' },
  templateLang: 'webc',
  css: 'vanilla',
  indieweb: { webmention: true, micropub: true, indieauth: true },
};

export const wizard = writable<WizardState>({ ...defaultState });

export const totalSteps = 6;

export const progress = derived(wizard, ($w) => $w.step / (totalSteps - 1));

export const canAdvance = derived(wizard, ($w) => {
  switch ($w.step) {
    case 0: return $w.identity.name.trim() !== '' && $w.identity.url.trim() !== '';
    case 1: return $w.site.name.trim() !== '' && $w.site.directory.trim() !== '';
    default: return true;
  }
});

export function nextStep() {
  wizard.update((s) => ({ ...s, step: Math.min(s.step + 1, totalSteps - 1) }));
}

export function prevStep() {
  wizard.update((s) => ({ ...s, step: Math.max(s.step - 1, 0) }));
}

export function goToStep(step: number) {
  wizard.update((s) => ({ ...s, step }));
}

export function resetWizard() {
  wizard.set({ ...defaultState });
}

export function updateIdentity(identity: Partial<WizardState['identity']>) {
  wizard.update((s) => ({ ...s, identity: { ...s.identity, ...identity } }));
}

export function updateSite(site: Partial<WizardState['site']>) {
  wizard.update((s) => ({ ...s, site: { ...s.site, ...site } }));
}

export function updateIndieweb(indieweb: Partial<WizardState['indieweb']>) {
  wizard.update((s) => ({ ...s, indieweb: { ...s.indieweb, ...indieweb } }));
}
