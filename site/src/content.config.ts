import { defineCollection } from 'astro:content';
import { docsSchema } from '@astrojs/starlight/schema';
import { publicDocsLoader } from './docs/loader';

export const collections = {
  docs: defineCollection({ loader: publicDocsLoader(), schema: docsSchema() }),
};
