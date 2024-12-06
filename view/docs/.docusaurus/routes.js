import React from "react";
import ComponentCreator from "@docusaurus/ComponentCreator";

export default [
  {
    path: "/moss/__docusaurus/debug",
    component: ComponentCreator("/moss/__docusaurus/debug", "01c"),
    exact: true,
  },
  {
    path: "/moss/__docusaurus/debug/config",
    component: ComponentCreator("/moss/__docusaurus/debug/config", "f85"),
    exact: true,
  },
  {
    path: "/moss/__docusaurus/debug/content",
    component: ComponentCreator("/moss/__docusaurus/debug/content", "65d"),
    exact: true,
  },
  {
    path: "/moss/__docusaurus/debug/globalData",
    component: ComponentCreator("/moss/__docusaurus/debug/globalData", "7ea"),
    exact: true,
  },
  {
    path: "/moss/__docusaurus/debug/metadata",
    component: ComponentCreator("/moss/__docusaurus/debug/metadata", "1b0"),
    exact: true,
  },
  {
    path: "/moss/__docusaurus/debug/registry",
    component: ComponentCreator("/moss/__docusaurus/debug/registry", "525"),
    exact: true,
  },
  {
    path: "/moss/__docusaurus/debug/routes",
    component: ComponentCreator("/moss/__docusaurus/debug/routes", "0df"),
    exact: true,
  },
  {
    path: "/moss/blog",
    component: ComponentCreator("/moss/blog", "0dc"),
    exact: true,
  },
  {
    path: "/moss/blog/archive",
    component: ComponentCreator("/moss/blog/archive", "918"),
    exact: true,
  },
  {
    path: "/moss/blog/first-blog-post",
    component: ComponentCreator("/moss/blog/first-blog-post", "735"),
    exact: true,
  },
  {
    path: "/moss/blog/long-blog-post",
    component: ComponentCreator("/moss/blog/long-blog-post", "ca5"),
    exact: true,
  },
  {
    path: "/moss/blog/mdx-blog-post",
    component: ComponentCreator("/moss/blog/mdx-blog-post", "161"),
    exact: true,
  },
  {
    path: "/moss/blog/tags",
    component: ComponentCreator("/moss/blog/tags", "11b"),
    exact: true,
  },
  {
    path: "/moss/blog/tags/docusaurus",
    component: ComponentCreator("/moss/blog/tags/docusaurus", "af6"),
    exact: true,
  },
  {
    path: "/moss/blog/tags/facebook",
    component: ComponentCreator("/moss/blog/tags/facebook", "ddb"),
    exact: true,
  },
  {
    path: "/moss/blog/tags/hello",
    component: ComponentCreator("/moss/blog/tags/hello", "a2a"),
    exact: true,
  },
  {
    path: "/moss/blog/tags/hola",
    component: ComponentCreator("/moss/blog/tags/hola", "ad8"),
    exact: true,
  },
  {
    path: "/moss/blog/welcome",
    component: ComponentCreator("/moss/blog/welcome", "90b"),
    exact: true,
  },
  {
    path: "/moss/markdown-page",
    component: ComponentCreator("/moss/markdown-page", "d6b"),
    exact: true,
  },
  {
    path: "/moss/design-system",
    component: ComponentCreator("/moss/design-system", "dde"),
    routes: [
      {
        path: "/moss/design-system",
        component: ComponentCreator("/moss/design-system", "dab"),
        routes: [
          {
            path: "/moss/design-system",
            component: ComponentCreator("/moss/design-system", "9c6"),
            routes: [
              {
                path: "/moss/design-system/",
                component: ComponentCreator("/moss/design-system/", "52f"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/design-system/category/tokens",
                component: ComponentCreator("/moss/design-system/category/tokens", "163"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/design-system/overview/all-tokens",
                component: ComponentCreator("/moss/design-system/overview/all-tokens", "8f0"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/design-system/overview/use-in-code",
                component: ComponentCreator("/moss/design-system/overview/use-in-code", "8d4"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/design-system/overview/use-in-design",
                component: ComponentCreator("/moss/design-system/overview/use-in-design", "eb4"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
            ],
          },
        ],
      },
    ],
  },
  {
    path: "/moss/docs",
    component: ComponentCreator("/moss/docs", "e3d"),
    routes: [
      {
        path: "/moss/docs",
        component: ComponentCreator("/moss/docs", "e4e"),
        routes: [
          {
            path: "/moss/docs",
            component: ComponentCreator("/moss/docs", "243"),
            routes: [
              {
                path: "/moss/docs/category/tutorial---basics",
                component: ComponentCreator("/moss/docs/category/tutorial---basics", "f42"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/category/tutorial---extras",
                component: ComponentCreator("/moss/docs/category/tutorial---extras", "584"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/intro",
                component: ComponentCreator("/moss/docs/intro", "f13"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-basics/congratulations",
                component: ComponentCreator("/moss/docs/tutorial-basics/congratulations", "69f"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-basics/create-a-blog-post",
                component: ComponentCreator("/moss/docs/tutorial-basics/create-a-blog-post", "afd"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-basics/create-a-document",
                component: ComponentCreator("/moss/docs/tutorial-basics/create-a-document", "fe2"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-basics/create-a-page",
                component: ComponentCreator("/moss/docs/tutorial-basics/create-a-page", "edd"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-basics/deploy-your-site",
                component: ComponentCreator("/moss/docs/tutorial-basics/deploy-your-site", "f47"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-basics/markdown-features",
                component: ComponentCreator("/moss/docs/tutorial-basics/markdown-features", "3ff"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-extras/manage-docs-versions",
                component: ComponentCreator("/moss/docs/tutorial-extras/manage-docs-versions", "3c6"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/docs/tutorial-extras/translate-your-site",
                component: ComponentCreator("/moss/docs/tutorial-extras/translate-your-site", "d2f"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
            ],
          },
        ],
      },
    ],
  },
  {
    path: "/moss/web",
    component: ComponentCreator("/moss/web", "ec4"),
    routes: [
      {
        path: "/moss/web",
        component: ComponentCreator("/moss/web", "e68"),
        routes: [
          {
            path: "/moss/web",
            component: ComponentCreator("/moss/web", "748"),
            routes: [
              {
                path: "/moss/web/",
                component: ComponentCreator("/moss/web/", "a25"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
              {
                path: "/moss/web/web_doc",
                component: ComponentCreator("/moss/web/web_doc", "de6"),
                exact: true,
                sidebar: "tutorialSidebar",
              },
            ],
          },
        ],
      },
    ],
  },
  {
    path: "/moss/",
    component: ComponentCreator("/moss/", "c1c"),
    exact: true,
  },
  {
    path: "*",
    component: ComponentCreator("*"),
  },
];
