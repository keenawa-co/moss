import { Suspense, useEffect, useRef } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { Home, Logs, Settings } from "@/pages";
import { useLayoutStore } from "@/store/layout";

import { LaunchPad } from "../components/LaunchPad";
import { Menu } from "../components/Menu";
import { Resizable, ResizablePanel } from "../components/Resizable";
import { ContentLayout } from "./ContentLayout";

export const AppLayout = () => {
  const alignment = useLayoutStore((state) => state.alignment);

  if (alignment === "center") {
    return <CenterLayout />;
  }
  if (alignment === "justify") {
    return <JustifyLayout />;
  }
  if (alignment === "left") {
    return <LeftLayout />;
  }
  if (alignment === "right") {
    return <RightLayout />;
  }
};

// Resizable layouts

const CenterLayout = () => {
  const { bottomPane, primarySideBar, secondarySideBar } = useLayoutStore((state) => state);

  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
        primarySideBar.setWidth(primarySideBarWidth);
        secondarySideBar.setWidth(secondarySideBarWidth);
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={primarySideBar.width}
        snap
        visible={primarySideBar.visibility}
        className="select-none"
      >
        <PrimarySideBar />
      </ResizablePanel>
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            bottomPane.setHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <ResizablePanel preferredSize={bottomPane.height} snap minSize={100} visible={bottomPane.visibility}>
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
      <ResizablePanel
        minSize={100}
        preferredSize={secondarySideBar.width}
        snap
        visible={secondarySideBar.visibility}
        className="select-none"
      >
        <SecondarySideBar />
      </ResizablePanel>
    </Resizable>
  );
};

const JustifyLayout = () => {
  const { bottomPane, primarySideBar, secondarySideBar } = useLayoutStore((state) => state);

  return (
    <Resizable
      vertical
      onDragEnd={(sizes) => {
        const [_, bottomPaneHeight] = sizes;
        bottomPane.setHeight(bottomPaneHeight);
      }}
    >
      <ResizablePanel>
        <Resizable
          onDragEnd={(sizes) => {
            const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
            primarySideBar.setWidth(primarySideBarWidth);
            secondarySideBar.setWidth(secondarySideBarWidth);
          }}
        >
          <ResizablePanel
            minSize={100}
            preferredSize={primarySideBar.width}
            snap
            visible={primarySideBar.visibility}
            className="select-none"
          >
            <PrimarySideBar />
          </ResizablePanel>
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <ResizablePanel
            minSize={100}
            preferredSize={secondarySideBar.width}
            snap
            visible={secondarySideBar.visibility}
          >
            <SecondarySideBar />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel preferredSize={bottomPane.height} snap minSize={100} visible={bottomPane.visibility}>
        <BottomPaneContent />
      </ResizablePanel>
    </Resizable>
  );
};

const LeftLayout = () => {
  const { bottomPane, primarySideBar, secondarySideBar } = useLayoutStore((state) => state);

  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [_, secondarySideBarWidth] = sizes;
        secondarySideBar.setWidth(secondarySideBarWidth);
      }}
    >
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            bottomPane.setHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [primarySideBarWidth, _] = sizes;
                primarySideBar.setWidth(primarySideBarWidth);
              }}
            >
              <ResizablePanel
                minSize={100}
                preferredSize={primarySideBar.width}
                snap
                visible={primarySideBar.visibility}
                className="select-none"
              >
                <PrimarySideBar />
              </ResizablePanel>
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          <ResizablePanel preferredSize={bottomPane.height} snap minSize={100} visible={bottomPane.visibility}>
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel minSize={100} preferredSize={secondarySideBar.width} snap visible={secondarySideBar.visibility}>
        <SecondarySideBar />
      </ResizablePanel>
    </Resizable>
  );
};

const RightLayout = () => {
  const { bottomPane, primarySideBar, secondarySideBar } = useLayoutStore((state) => state);

  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _] = sizes;
        primarySideBar.setWidth(primarySideBarWidth);
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={primarySideBar.width}
        snap
        visible={primarySideBar.visibility}
        className="select-none"
      >
        <PrimarySideBar />
      </ResizablePanel>

      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            bottomPane.setHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [_, secondarySideBarWidth] = sizes;
                secondarySideBar.setWidth(secondarySideBarWidth);
              }}
            >
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              <ResizablePanel
                minSize={100}
                preferredSize={secondarySideBar.width}
                snap
                visible={secondarySideBar.visibility}
              >
                <SecondarySideBar />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          <ResizablePanel preferredSize={bottomPane.height} snap minSize={100} visible={bottomPane.visibility}>
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
    </Resizable>
  );
};

//-----------------------------------

const PrimarySideBar = () => <LaunchPad />;

const SecondarySideBar = () => <div>SecondarySideBar</div>;

const MainContent = () => (
  <ContentLayout className="relative flex h-full flex-col overflow-auto">
    <Suspense fallback={<div>Loading...</div>}>
      <BrowserRouter>
        <Menu />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/logs" element={<Logs />} />
        </Routes>
      </BrowserRouter>
    </Suspense>
  </ContentLayout>
);

const BottomPaneContent = () => (
  <div className="h-full overflow-auto">
    <div className="mt-5">
      <div>List of 50 elements:</div>
      {Array.from({ length: 50 }).map((_, i) => (
        <div key={i}>{i + 1 === 50 ? "last element" : `${i + 1}: ${Math.random().toFixed(2)}`}</div>
      ))}
    </div>
  </div>
);
