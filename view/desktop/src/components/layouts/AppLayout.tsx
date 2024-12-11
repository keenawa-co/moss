import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { Home, Logs, Settings } from "@/pages";
import { useLayoutStore } from "@/store/layoutStore";

import { LaunchPad } from "../LaunchPad";
import { Menu } from "../Menu";
import { Resizable, ResizablePanel } from "../Resizable";
import { ContentLayout } from "./ContentLayout";

export const AppLayout = () => {
  const primarySideBarVisibility = useLayoutStore((state) => state.primarySideBar.visibility);
  const setPrimarySideBarWidth = useLayoutStore((state) => state.primarySideBar.setWidth);

  const bottomPaneVisibility = useLayoutStore((state) => state.bottomPane.visibility);
  const setBottomPaneHeight = useLayoutStore((state) => state.bottomPane.setHeight);

  return (
    <Resizable
      proportionalLayout={false}
      onDragEnd={(sizes) => {
        setPrimarySideBarWidth(sizes[0]);
      }}
    >
      <ResizablePanel minSize={100} preferredSize={255} snap visible={primarySideBarVisibility} className="select-none">
        <LaunchPad />
      </ResizablePanel>
      <ResizablePanel>
        <Resizable
          vertical
          onDragEnd={(sizes) => {
            setBottomPaneHeight(sizes[1]);
          }}
        >
          <ResizablePanel>
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
          </ResizablePanel>
          <ResizablePanel preferredSize={144} snap minSize={100} maxSize={500} visible={bottomPaneVisibility}>
            <div className="h-full overflow-auto">
              <div>
                <div>List of 50 elements:</div>
                {Array.from({ length: 50 }).map((_, i) => (
                  <div key={i}>{i + 1 === 50 ? "last element" : i + 1}</div>
                ))}
              </div>
            </div>
          </ResizablePanel>
        </Resizable>
      </ResizablePanel>
    </Resizable>
  );
};
