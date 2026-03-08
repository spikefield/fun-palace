import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ProjectEntry } from '$lib/types';

export const projects = writable<ProjectEntry[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export async function loadProjects() {
  loading.set(true);
  error.set(null);
  try {
    const result = await invoke<ProjectEntry[]>('list_projects');
    projects.set(result);
  } catch (e: unknown) {
    const err = e as { message?: string };
    error.set(err.message ?? 'Failed to load projects');
  } finally {
    loading.set(false);
  }
}

export async function removeProject(id: string) {
  await invoke('remove_project', { id });
  await loadProjects();
}
