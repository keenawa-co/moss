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
  const primarySideBarVisibility = useLayoutStore((state) => state.primarySideBar.visibility);
  const primarySideBarSetWidth = useLayoutStore((state) => state.primarySideBar.setWidth);
  const primarySideBarGetWidth = useLayoutStore((state) => state.primarySideBar.getWidth);

  const secondarySideBarVisibility = useLayoutStore((state) => state.secondarySideBar.visibility);
  const secondarySideBarSetWidth = useLayoutStore((state) => state.secondarySideBar.setWidth);
  const secondarySideBarGetWidth = useLayoutStore((state) => state.secondarySideBar.getWidth);

  const bottomPaneVisibility = useLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useLayoutStore((state) => state.bottomPane.getHeight);

  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
        primarySideBarSetWidth(primarySideBarWidth);
        secondarySideBarSetWidth(secondarySideBarWidth);
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={primarySideBarGetWidth()}
        snap
        visible={primarySideBarVisibility}
        className="select-none"
      >
        <PrimarySideBar />
      </ResizablePanel>
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            bottomPaneSetHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <ResizablePanel preferredSize={bottomPaneGetHeight()} snap minSize={100} visible={bottomPaneVisibility}>
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
      <ResizablePanel
        minSize={100}
        preferredSize={secondarySideBarGetWidth()}
        snap
        visible={secondarySideBarVisibility}
        className="select-none"
      >
        <SecondarySideBar />
      </ResizablePanel>
    </Resizable>
  );
};

const JustifyLayout = () => {
  const primarySideBarVisibility = useLayoutStore((state) => state.primarySideBar.visibility);
  const primarySideBarSetWidth = useLayoutStore((state) => state.primarySideBar.setWidth);
  const primarySideBarGetWidth = useLayoutStore((state) => state.primarySideBar.getWidth);

  const secondarySideBarVisibility = useLayoutStore((state) => state.secondarySideBar.visibility);
  const secondarySideBarSetWidth = useLayoutStore((state) => state.secondarySideBar.setWidth);
  const secondarySideBarGetWidth = useLayoutStore((state) => state.secondarySideBar.getWidth);

  const bottomPaneVisibility = useLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useLayoutStore((state) => state.bottomPane.getHeight);

  return (
    <Resizable
      vertical
      onDragEnd={(sizes) => {
        const [_, bottomPaneHeight] = sizes;
        bottomPaneSetHeight(bottomPaneHeight);
      }}
    >
      <ResizablePanel>
        <Resizable
          onDragEnd={(sizes) => {
            const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
            primarySideBarSetWidth(primarySideBarWidth);
            secondarySideBarSetWidth(secondarySideBarWidth);
          }}
        >
          <ResizablePanel
            minSize={100}
            preferredSize={primarySideBarGetWidth()}
            snap
            visible={primarySideBarVisibility}
            className="select-none"
          >
            <PrimarySideBar />
          </ResizablePanel>
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <ResizablePanel
            minSize={100}
            preferredSize={secondarySideBarGetWidth()}
            snap
            visible={secondarySideBarVisibility}
          >
            <SecondarySideBar />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel preferredSize={bottomPaneGetHeight()} snap minSize={100} visible={bottomPaneVisibility}>
        <BottomPaneContent />
      </ResizablePanel>
    </Resizable>
  );
};

const LeftLayout = () => {
  const primarySideBarVisibility = useLayoutStore((state) => state.primarySideBar.visibility);
  const primarySideBarSetWidth = useLayoutStore((state) => state.primarySideBar.setWidth);
  const primarySideBarGetWidth = useLayoutStore((state) => state.primarySideBar.getWidth);

  const secondarySideBarVisibility = useLayoutStore((state) => state.secondarySideBar.visibility);
  const secondarySideBarSetWidth = useLayoutStore((state) => state.secondarySideBar.setWidth);
  const secondarySideBarGetWidth = useLayoutStore((state) => state.secondarySideBar.getWidth);

  const bottomPaneVisibility = useLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useLayoutStore((state) => state.bottomPane.getHeight);

  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [_, secondarySideBarWidth] = sizes;
        secondarySideBarSetWidth(secondarySideBarWidth);
      }}
    >
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            bottomPaneSetHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [primarySideBarWidth, _] = sizes;
                primarySideBarSetWidth(primarySideBarWidth);
              }}
            >
              <ResizablePanel
                minSize={100}
                preferredSize={primarySideBarGetWidth()}
                snap
                visible={primarySideBarVisibility}
                className="select-none"
              >
                <PrimarySideBar />
              </ResizablePanel>
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          <ResizablePanel preferredSize={bottomPaneGetHeight()} snap minSize={100} visible={bottomPaneVisibility}>
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel
        minSize={100}
        preferredSize={secondarySideBarGetWidth()}
        snap
        visible={secondarySideBarVisibility}
      >
        <SecondarySideBar />
      </ResizablePanel>
    </Resizable>
  );
};

const RightLayout = () => {
  const primarySideBarVisibility = useLayoutStore((state) => state.primarySideBar.visibility);
  const primarySideBarSetWidth = useLayoutStore((state) => state.primarySideBar.setWidth);
  const primarySideBarGetWidth = useLayoutStore((state) => state.primarySideBar.getWidth);

  const secondarySideBarVisibility = useLayoutStore((state) => state.secondarySideBar.visibility);
  const secondarySideBarSetWidth = useLayoutStore((state) => state.secondarySideBar.setWidth);
  const secondarySideBarGetWidth = useLayoutStore((state) => state.secondarySideBar.getWidth);

  const bottomPaneVisibility = useLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useLayoutStore((state) => state.bottomPane.getHeight);

  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _] = sizes;
        primarySideBarSetWidth(primarySideBarWidth);
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={primarySideBarGetWidth()}
        snap
        visible={primarySideBarVisibility}
        className="select-none"
      >
        <PrimarySideBar />
      </ResizablePanel>

      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            bottomPaneSetHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [_, secondarySideBarWidth] = sizes;
                secondarySideBarSetWidth(secondarySideBarWidth);
              }}
            >
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              <ResizablePanel
                minSize={100}
                preferredSize={secondarySideBarGetWidth()}
                snap
                visible={secondarySideBarVisibility}
              >
                <SecondarySideBar />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          <ResizablePanel preferredSize={bottomPaneGetHeight()} snap minSize={100} visible={bottomPaneVisibility}>
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
    </Resizable>
  );
};

//----------------------------------------------------------------------------------------------

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
