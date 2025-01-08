import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { AppLayoutState, useChangeAppLayoutState, useGetAppLayoutState } from "@/hooks/useAppLayoutState";
import { Home, Logs, Settings } from "@/pages";

import { LaunchPad } from "../components/LaunchPad";
import { Menu } from "../components/Menu";
import { Resizable, ResizablePanel } from "../components/Resizable";
import { ContentLayout } from "./ContentLayout";

export const AppLayout = () => {
  const { data: layout } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();

  if (!layout) return null;

  if (layout.alignment === "center") {
    return <CenterLayout layout={layout} changeAppLayoutState={changeAppLayoutState} />;
  }
  if (layout.alignment === "justify") {
    return <JustifyLayout layout={layout} changeAppLayoutState={changeAppLayoutState} />;
  }
  if (layout.alignment === "left") {
    return <LeftLayout layout={layout} changeAppLayoutState={changeAppLayoutState} />;
  }
  if (layout.alignment === "right") {
    return <RightLayout layout={layout} changeAppLayoutState={changeAppLayoutState} />;
  }
};

// Resizable layouts

interface ResizableLayoutProps {
  layout: AppLayoutState;
  changeAppLayoutState: (newLayout: AppLayoutState) => void;
}

const CenterLayout = ({ layout, changeAppLayoutState }: ResizableLayoutProps) => {
  return (
    <Resizable
      key={layout.primarySideBarPosition}
      proportionalLayout
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
        changeAppLayoutState({
          ...layout,
          primarySideBar: {
            ...layout.primarySideBar,
            width: layout.primarySideBarPosition === "left" ? primarySideBarWidth : secondarySideBarWidth,
          },
          secondarySideBar: {
            ...layout.secondarySideBar,
            width: layout.primarySideBarPosition === "left" ? secondarySideBarWidth : primarySideBarWidth,
          },
        });
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={
          layout.primarySideBarPosition === "left" ? layout.primarySideBar.width : layout.secondarySideBar.width
        }
        snap
        visible={
          layout.primarySideBarPosition === "left"
            ? layout.primarySideBar.visibility
            : layout.secondarySideBar.visibility
        }
        className="select-none"
      >
        {layout.primarySideBarPosition === "left" ? <PrimarySideBar /> : <SecondarySideBar />}
      </ResizablePanel>
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            layout.bottomPane.height = bottomPaneHeight;
          }}
        >
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <ResizablePanel
            preferredSize={layout.bottomPane.height}
            snap
            minSize={100}
            visible={layout.bottomPane.visibility}
          >
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
      <ResizablePanel
        minSize={100}
        preferredSize={
          layout.primarySideBarPosition === "right" ? layout.primarySideBar.width : layout.secondarySideBar.width
        }
        snap
        visible={
          layout.primarySideBarPosition === "right"
            ? layout.primarySideBar.visibility
            : layout.secondarySideBar.visibility
        }
        className="select-none"
      >
        {layout.primarySideBarPosition === "right" ? <PrimarySideBar /> : <SecondarySideBar />}
      </ResizablePanel>
    </Resizable>
  );
};

const JustifyLayout = ({ layout, changeAppLayoutState }: ResizableLayoutProps) => {
  return (
    <Resizable
      vertical
      onDragEnd={(sizes) => {
        const [_, bottomPaneHeight] = sizes;
        layout.bottomPane.height = bottomPaneHeight;
      }}
    >
      <ResizablePanel>
        <Resizable
          onDragEnd={(sizes) => {
            const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
            layout.primarySideBar.width = primarySideBarWidth;
            layout.secondarySideBar.width = secondarySideBarWidth;
          }}
        >
          <ResizablePanel
            minSize={100}
            preferredSize={layout.primarySideBar.width}
            snap
            visible={layout.primarySideBar.visibility}
            className="select-none"
          >
            <PrimarySideBar />
          </ResizablePanel>
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <ResizablePanel
            minSize={100}
            preferredSize={layout.secondarySideBar.width}
            snap
            visible={layout.secondarySideBar.visibility}
          >
            <SecondarySideBar />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel
        preferredSize={layout.bottomPane.height}
        snap
        minSize={100}
        visible={layout.bottomPane.visibility}
      >
        <BottomPaneContent />
      </ResizablePanel>
    </Resizable>
  );
};

const LeftLayout = ({ layout, changeAppLayoutState }: ResizableLayoutProps) => {
  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [_, secondarySideBarWidth] = sizes;
        layout.secondarySideBar.width = secondarySideBarWidth;
      }}
    >
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            layout.bottomPane.height = bottomPaneHeight;
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [primarySideBarWidth, _] = sizes;
                layout.primarySideBar.width = primarySideBarWidth;
              }}
            >
              <ResizablePanel
                minSize={100}
                preferredSize={layout.primarySideBar.width}
                snap
                visible={layout.primarySideBar.visibility}
                className="select-none"
              >
                <PrimarySideBar />
              </ResizablePanel>
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          <ResizablePanel
            preferredSize={layout.bottomPane.height}
            snap
            minSize={100}
            visible={layout.bottomPane.visibility}
          >
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel
        minSize={100}
        preferredSize={layout.secondarySideBar.width}
        snap
        visible={layout.secondarySideBar.visibility}
      >
        <SecondarySideBar />
      </ResizablePanel>
    </Resizable>
  );
};

const RightLayout = ({ layout, changeAppLayoutState }: ResizableLayoutProps) => {
  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _] = sizes;
        layout.primarySideBar.width = primarySideBarWidth;
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={layout.primarySideBar.width}
        snap
        visible={layout.primarySideBar.visibility}
        className="select-none"
      >
        <PrimarySideBar />
      </ResizablePanel>

      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            layout.bottomPane.height = bottomPaneHeight;
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [_, secondarySideBarWidth] = sizes;
                layout.secondarySideBar.width = secondarySideBarWidth;
              }}
            >
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              <ResizablePanel
                minSize={100}
                preferredSize={layout.secondarySideBar.width}
                snap
                visible={layout.secondarySideBar.visibility}
              >
                <SecondarySideBar />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          <ResizablePanel
            preferredSize={layout.bottomPane.height}
            snap
            minSize={100}
            visible={layout.bottomPane.visibility}
          >
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
    </Resizable>
  );
};

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
