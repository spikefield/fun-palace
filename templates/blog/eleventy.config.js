import { feedPlugin } from "@11ty/eleventy-plugin-rss";

export default function (eleventyConfig) {
  const siteData = {
    title: "My Site",
    url: "https://example.com",
    author: "Your Name",
  };

  eleventyConfig.addPlugin(feedPlugin, {
    type: "atom",
    outputPath: "/feed.xml",
    collection: { name: "posts", limit: 20 },
    metadata: {
      language: "en",
      title: siteData.title,
      subtitle: "",
      base: siteData.url,
      author: siteData.author,
    },
  });

  eleventyConfig.addPlugin(feedPlugin, {
    type: "json",
    outputPath: "/feed.json",
    collection: { name: "posts", limit: 20 },
    metadata: {
      language: "en",
      title: siteData.title,
      subtitle: "",
      base: siteData.url,
      author: siteData.author,
    },
  });

  return {
    dir: {
      input: "src",
      output: "_site",
      includes: "_includes",
      data: "_data",
    },
    templateFormats: ["md", "njk", "html"],
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
  };
}
