import { LayoutPriority } from "allotment";
import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { AppLayoutState, useGetAppLayoutState } from "@/hooks/useAppLayoutState";
import { Home, Logs, Settings } from "@/pages";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { LaunchPad } from "../components/LaunchPad";
import { Menu } from "../components/Menu";
import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";
import { ContentLayout } from "./ContentLayout";

export const AppLayout = () => {
  const { data: appLayoutState } = useGetAppLayoutState();

  const primarySideBarVisibility = useAppResizableLayoutStore((state) => state.primarySideBar.visibility);
  const primarySideBarSetWidth = useAppResizableLayoutStore((state) => state.primarySideBar.setWidth);
  const primarySideBarGetWidth = useAppResizableLayoutStore((state) => state.primarySideBar.getWidth);

  const secondarySideBarVisibility = useAppResizableLayoutStore((state) => state.secondarySideBar.visibility);
  const secondarySideBarSetWidth = useAppResizableLayoutStore((state) => state.secondarySideBar.setWidth);
  const secondarySideBarGetWidth = useAppResizableLayoutStore((state) => state.secondarySideBar.getWidth);

  const bottomPaneVisibility = useAppResizableLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useAppResizableLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useAppResizableLayoutStore((state) => state.bottomPane.getHeight);

  if (appLayoutState?.alignment === "center") {
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
        primarySideBarPosition={appLayoutState?.primarySideBarPosition}
      />
    );
  }
  if (appLayoutState?.alignment === "justify") {
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
        primarySideBarPosition={appLayoutState?.primarySideBarPosition}
      />
    );
  }
  if (appLayoutState?.alignment === "left") {
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
        primarySideBarPosition={appLayoutState?.primarySideBarPosition}
      />
    );
  }
  if (appLayoutState?.alignment === "right") {
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
        primarySideBarPosition={appLayoutState?.primarySideBarPosition}
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
  primarySideBarPosition: AppLayoutState["primarySideBarPosition"];
}

const CenterLayout = ({ ...props }: ResizableLayoutProps) => {
  return (
    <Resizable
      key={props.primarySideBarPosition}
      proportionalLayout={false}
      onDragEnd={(sizes) => {
        const [primarySideBarWidth, _, secondarySideBarWidth] = sizes;
        props.primarySideBarSetWidth(
          props.primarySideBarPosition === "left" ? primarySideBarWidth : secondarySideBarWidth
        );
        props.secondarySideBarSetWidth(
          props.primarySideBarPosition === "left" ? secondarySideBarWidth : primarySideBarWidth
        );
      }}
    >
      <ResizablePanel
        priority={LayoutPriority["Normal"]}
        minSize={100}
        preferredSize={
          props.primarySideBarPosition === "left" ? props.primarySideBarGetWidth() : props.secondarySideBarGetWidth()
        }
        snap
        visible={
          props.primarySideBarPosition === "left" ? props.primarySideBarVisibility : props.secondarySideBarVisibility
        }
        className="select-none"
      >
        {props.primarySideBarPosition === "left" ? <PrimarySideBar /> : <SecondarySideBar />}
      </ResizablePanel>
      <ResizablePanel priority={LayoutPriority["High"]}>
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
          <TabbedPane theme="dockview-theme-light" />
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
        priority={LayoutPriority["Normal"]}
        minSize={100}
        preferredSize={
          props.primarySideBarPosition === "right" ? props.primarySideBarGetWidth() : props.secondarySideBarGetWidth()
        }
        snap
        visible={
          props.primarySideBarPosition === "right" ? props.primarySideBarVisibility : props.secondarySideBarVisibility
        }
        className="select-none"
      >
        {props.primarySideBarPosition === "right" ? <PrimarySideBar /> : <SecondarySideBar />}
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
