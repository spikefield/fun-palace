export interface ProjectEntry {
  id: string;
  name: string;
  path: string;
  created_at: string;
  template_lang: string;
  css_approach: string;
}

export interface ScaffoldOptions {
  name: string;
  directory: string;
  starter: string;
  template_lang: string;
  css: string;
  author_name: string;
  author_url: string;
  site_url: string;
}

export interface TwelvetyError {
  code: string;
  message: string;
}
