export default {
  name: "My Site",
  eleventyVersion: "3.0",
  templateLang: "nunjucks",
  css: "vanilla",
  deploy: {
    target: "github-pages",
  },
  indieweb: {
    domain: "example.com",
    author: {
      name: "Your Name",
      url: "https://example.com",
      photo: "/img/avatar.jpg",
    },
    micropub: false,
    webmention: false,
  },
};
