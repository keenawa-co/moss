import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { Home, Logs, Settings } from "@/pages";
import { useLayoutStore } from "@/store/layout";

import "@repo/moss-tabs/styles/dockview.css";

//import "./moss-tabs-demo/styles.css";

import { LaunchPad } from "../components/LaunchPad";
import { Menu } from "../components/Menu";
import { Resizable, ResizablePanel } from "../components/Resizable";
import { ContentLayout } from "./ContentLayout";
import DockviewDemo from "./moss-tabs-demo/DockviewDemo";

export const AppLayout = () => {
  const alignment = useLayoutStore((state) => state.alignment);

  const primarySideBarVisibility = useLayoutStore((state) => state.primarySideBar.visibility);
  const primarySideBarSetWidth = useLayoutStore((state) => state.primarySideBar.setWidth);
  const primarySideBarGetWidth = useLayoutStore((state) => state.primarySideBar.getWidth);

  const secondarySideBarVisibility = useLayoutStore((state) => state.secondarySideBar.visibility);
  const secondarySideBarSetWidth = useLayoutStore((state) => state.secondarySideBar.setWidth);
  const secondarySideBarGetWidth = useLayoutStore((state) => state.secondarySideBar.getWidth);

  const bottomPaneVisibility = useLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useLayoutStore((state) => state.bottomPane.getHeight);

  if (alignment === "center") {
    return (
      <CenterLayout
        primarySideBarVisibility={primarySideBarVisibility}
        primarySideBarSetWidth={primarySideBarSetWidth}
        primarySideBarGetWidth={primarySideBarGetWidth}
        secondarySideBarVisibility={secondarySideBarVisibility}
        secondarySideBarSetWidth={secondarySideBarSetWidth}
        secondarySideBarGetWidth={secondarySideBarGetWidth}
        bottomPaneVisibility={bottomPaneVisibility}
        bottomPaneSetHeight={bottomPaneSetHeight}
        bottomPaneGetHeight={bottomPaneGetHeight}
      />
    );
  }
  if (alignment === "justify") {
    return (
      <JustifyLayout
        primarySideBarVisibility={primarySideBarVisibility}
        primarySideBarSetWidth={primarySideBarSetWidth}
        primarySideBarGetWidth={primarySideBarGetWidth}
        secondarySideBarVisibility={secondarySideBarVisibility}
        secondarySideBarSetWidth={secondarySideBarSetWidth}
        secondarySideBarGetWidth={secondarySideBarGetWidth}
        bottomPaneVisibility={bottomPaneVisibility}
        bottomPaneSetHeight={bottomPaneSetHeight}
        bottomPaneGetHeight={bottomPaneGetHeight}
      />
    );
  }
  if (alignment === "left") {
    return (
      <LeftLayout
        primarySideBarVisibility={primarySideBarVisibility}
        primarySideBarSetWidth={primarySideBarSetWidth}
        primarySideBarGetWidth={primarySideBarGetWidth}
        secondarySideBarVisibility={secondarySideBarVisibility}
        secondarySideBarSetWidth={secondarySideBarSetWidth}
        secondarySideBarGetWidth={secondarySideBarGetWidth}
        bottomPaneVisibility={bottomPaneVisibility}
        bottomPaneSetHeight={bottomPaneSetHeight}
        bottomPaneGetHeight={bottomPaneGetHeight}
      />
    );
  }
  if (alignment === "right") {
    return (
      <RightLayout
        primarySideBarVisibility={primarySideBarVisibility}
        primarySideBarSetWidth={primarySideBarSetWidth}
        primarySideBarGetWidth={primarySideBarGetWidth}
        secondarySideBarVisibility={secondarySideBarVisibility}
        secondarySideBarSetWidth={secondarySideBarSetWidth}
        secondarySideBarGetWidth={secondarySideBarGetWidth}
        bottomPaneVisibility={bottomPaneVisibility}
        bottomPaneSetHeight={bottomPaneSetHeight}
        bottomPaneGetHeight={bottomPaneGetHeight}
      />
    );
  }
};

// Resizable layouts

interface ResizableLayoutProps {
  primarySideBarVisibility: boolean;
  primarySideBarSetWidth: (newWidth: number) => void;
  primarySideBarGetWidth: () => number;
  secondarySideBarVisibility: boolean;
  secondarySideBarSetWidth: (newWidth: number) => void;
  secondarySideBarGetWidth: () => number;
  bottomPaneVisibility: boolean;
  bottomPaneSetHeight: (newHeight: number) => void;
  bottomPaneGetHeight: () => number;
}

const CenterLayout = ({ ...props }: ResizableLayoutProps) => {
  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
        props.primarySideBarSetWidth(primarySideBarWidth);
        props.secondarySideBarSetWidth(secondarySideBarWidth);
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={props.primarySideBarGetWidth()}
        snap
        visible={props.primarySideBarVisibility}
        className="select-none"
      >
        <PrimarySideBar />
      </ResizablePanel>
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            props.bottomPaneSetHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <DockviewDemo theme="dockview-theme-abyss" />
          <ResizablePanel
            preferredSize={props.bottomPaneGetHeight()}
            snap
            minSize={100}
            visible={props.bottomPaneVisibility}
          >
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
      <ResizablePanel
        minSize={100}
        preferredSize={props.secondarySideBarGetWidth()}
        snap
        visible={props.secondarySideBarVisibility}
        className="select-none"
      >
        <SecondarySideBar />
      </ResizablePanel>
    </Resizable>
  );
};

const JustifyLayout = ({ ...props }: ResizableLayoutProps) => {
  return (
    <Resizable
      vertical
      onDragEnd={(sizes) => {
        const [_, bottomPaneHeight] = sizes;
        props.bottomPaneSetHeight(bottomPaneHeight);
      }}
    >
      <ResizablePanel>
        <Resizable
          onDragEnd={(sizes) => {
            const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
            props.primarySideBarSetWidth(primarySideBarWidth);
            props.secondarySideBarSetWidth(secondarySideBarWidth);
          }}
        >
          <ResizablePanel
            minSize={100}
            preferredSize={props.primarySideBarGetWidth()}
            snap
            visible={props.primarySideBarVisibility}
            className="select-none"
          >
            <PrimarySideBar />
          </ResizablePanel>
          <ResizablePanel>
            <MainContent />
          </ResizablePanel>
          <ResizablePanel
            minSize={100}
            preferredSize={props.secondarySideBarGetWidth()}
            snap
            visible={props.secondarySideBarVisibility}
          >
            <SecondarySideBar />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel
        preferredSize={props.bottomPaneGetHeight()}
        snap
        minSize={100}
        visible={props.bottomPaneVisibility}
      >
        <BottomPaneContent />
      </ResizablePanel>
    </Resizable>
  );
};

const LeftLayout = ({ ...props }: ResizableLayoutProps) => {
  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [_, secondarySideBarWidth] = sizes;
        props.secondarySideBarSetWidth(secondarySideBarWidth);
      }}
    >
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            props.bottomPaneSetHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [primarySideBarWidth, _] = sizes;
                props.primarySideBarSetWidth(primarySideBarWidth);
              }}
            >
              <ResizablePanel
                minSize={100}
                preferredSize={props.primarySideBarGetWidth()}
                snap
                visible={props.primarySideBarVisibility}
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
            preferredSize={props.bottomPaneGetHeight()}
            snap
            minSize={100}
            visible={props.bottomPaneVisibility}
          >
            <BottomPaneContent />
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>

      <ResizablePanel
        minSize={100}
        preferredSize={props.secondarySideBarGetWidth()}
        snap
        visible={props.secondarySideBarVisibility}
      >
        <SecondarySideBar />
      </ResizablePanel>
    </Resizable>
  );
};

const RightLayout = ({ ...props }: ResizableLayoutProps) => {
  return (
    <Resizable
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _] = sizes;
        props.primarySideBarSetWidth(primarySideBarWidth);
      }}
    >
      <ResizablePanel
        minSize={100}
        preferredSize={props.primarySideBarGetWidth()}
        snap
        visible={props.primarySideBarVisibility}
        className="select-none"
      >
        <PrimarySideBar />
      </ResizablePanel>

      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            const [_, bottomPaneHeight] = sizes;
            props.bottomPaneSetHeight(bottomPaneHeight);
          }}
        >
          <ResizablePanel>
            <Resizable
              onDragEnd={(sizes) => {
                const [_, secondarySideBarWidth] = sizes;
                props.secondarySideBarSetWidth(secondarySideBarWidth);
              }}
            >
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              <ResizablePanel
                minSize={100}
                preferredSize={props.secondarySideBarGetWidth()}
                snap
                visible={props.secondarySideBarVisibility}
              >
                <SecondarySideBar />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          <ResizablePanel
            preferredSize={props.bottomPaneGetHeight()}
            snap
            minSize={100}
            visible={props.bottomPaneVisibility}
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
