import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

const config: Config = {
  title: "Moss Studio",
  tagline: "Dinosaurs are cool",
  favicon: "img/favicon.ico",
  url: "https://4rchr4y.github.io",
  baseUrl: "/moss",
  organizationName: "4rchr4y",
  projectName: "moss",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  deploymentBranch: "MOSSMVP-101-DevOps-Prepare-the-publication-of-the-documentation-site-on-GitHub-pages",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: "https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/",
        },
        blog: {},
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],
  plugins: [
    [
      "@docusaurus/plugin-content-docs",
      {
        id: "design-system",
        path: "./design-system",
        routeBasePath: "design-system",
        sidebarPath: "./sidebars.ts",
      },
    ],
    [
      "@docusaurus/plugin-content-docs",
      {
        id: "web",
        path: "../web/docs",
        routeBasePath: "web",
        sidebarPath: "./sidebars.ts",
      },
    ],
    // NOTE: commented-out code left as an example
    // [
    //   "@docusaurus/plugin-content-docs",
    //   {
    //     id: "shared",
    //     path: "../shared/docs",
    //     routeBasePath: "shared",
    //     sidebarPath: "./sidebars.ts",
    //   },
    // ],
  ],
  themeConfig: {
    navbar: {
      title: "My Site",
      logo: {
        alt: "My Site Logo",
        src: "img/logo.svg",
      },
      items: [
        // NOTE: commented-out code left as an example
        // {
        //   type: "docSidebar",
        //   sidebarId: "tutorialSidebar",
        //   position: "left",
        //   label: "Tutorial",
        // },
        { to: "/design-system", label: "Design System", position: "left" },
        {
          href: "https://github.com/4rchr4y/moss",
          label: "Moss",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            {
              label: "Design System",
              to: "/design-system",
            },
          ],
        },
        // NOTE: commented-out code left as an example
        // {
        //   title: "Community",
        //   items: [
        //     {
        //       label: "Stack Overflow",
        //       href: "https://stackoverflow.com/questions/tagged/docusaurus",
        //     },
        //     {
        //       label: "Discord",
        //       href: "https://discordapp.com/invite/docusaurus",
        //     },
        //     {
        //       label: "Twitter",
        //       href: "https://twitter.com/docusaurus",
        //     },
        //   ],
        // },
        // {
        //   title: "More",
        //   items: [
        //     {
        //       label: "Blog",
        //       to: "/blog",
        //     },
        //     {
        //       label: "GitHub",
        //       href: "https://github.com/facebook/docusaurus",
        //     },
        //   ],
        // },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} My Project, Inc. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
