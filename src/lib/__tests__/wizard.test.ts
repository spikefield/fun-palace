import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import {
  wizard,
  nextStep,
  prevStep,
  goToStep,
  resetWizard,
  updateIdentity,
  updateSite,
  progress,
  totalSteps,
} from '$lib/stores/wizard';

describe('wizard store', () => {
  beforeEach(() => resetWizard());

  it('starts at step 0', () => {
    expect(get(wizard).step).toBe(0);
  });

  it('nextStep increments', () => {
    nextStep();
    expect(get(wizard).step).toBe(1);
  });

  it('prevStep decrements', () => {
    nextStep();
    nextStep();
    prevStep();
    expect(get(wizard).step).toBe(1);
  });

  it('prevStep does not go below 0', () => {
    prevStep();
    expect(get(wizard).step).toBe(0);
  });

  it('nextStep does not exceed max', () => {
    for (let i = 0; i < totalSteps + 5; i++) nextStep();
    expect(get(wizard).step).toBe(totalSteps - 1);
  });

  it('goToStep sets step directly', () => {
    goToStep(3);
    expect(get(wizard).step).toBe(3);
  });

  it('updateIdentity merges partial updates', () => {
    updateIdentity({ name: 'Alice' });
    expect(get(wizard).identity.name).toBe('Alice');
    expect(get(wizard).identity.url).toBe('');
  });

  it('updateSite merges partial updates', () => {
    updateSite({ name: 'My Blog' });
    expect(get(wizard).site.name).toBe('My Blog');
  });

  it('progress is 0 at start', () => {
    expect(get(progress)).toBe(0);
  });

  it('resetWizard returns to defaults', () => {
    nextStep();
    updateIdentity({ name: 'Alice' });
    resetWizard();
    expect(get(wizard).step).toBe(0);
    expect(get(wizard).identity.name).toBe('');
  });
});
