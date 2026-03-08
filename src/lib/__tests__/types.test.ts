import { describe, it, expect } from 'vitest';
import type { ProjectEntry, ScaffoldOptions } from '$lib/types';

describe('types', () => {
  it('ProjectEntry shape is correct', () => {
    const project: ProjectEntry = {
      id: '123',
      name: 'Test',
      path: '/tmp/test',
      created_at: '2026-03-08',
      template_lang: 'nunjucks',
      css_approach: 'vanilla',
    };
    expect(project.id).toBe('123');
    expect(project.name).toBe('Test');
  });

  it('ScaffoldOptions shape is correct', () => {
    const opts: ScaffoldOptions = {
      name: 'Blog',
      directory: '/tmp/blog',
      starter: 'blog',
      template_lang: 'nunjucks',
      css: 'vanilla',
      author_name: 'Alice',
      author_url: 'https://alice.example',
      site_url: 'https://alice.example',
    };
    expect(opts.starter).toBe('blog');
  });
});
