import { readFileSync } from "fs";
import { feedPlugin } from "@11ty/eleventy-plugin-rss";

export default function (eleventyConfig) {
  const siteData = JSON.parse(readFileSync("src/_data/site.json", "utf-8"));

  eleventyConfig.addPlugin(feedPlugin, {
    type: "atom",
    outputPath: "/feed.xml",
    collection: { name: "posts", limit: 20 },
    metadata: {
      language: siteData.language || "en",
      title: siteData.title,
      subtitle: siteData.description || "",
      base: siteData.url,
      author: siteData.author?.name || siteData.author,
    },
  });

  eleventyConfig.addPlugin(feedPlugin, {
    type: "json",
    outputPath: "/feed.json",
    collection: { name: "posts", limit: 20 },
    metadata: {
      language: siteData.language || "en",
      title: siteData.title,
      subtitle: siteData.description || "",
      base: siteData.url,
      author: siteData.author?.name || siteData.author,
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
