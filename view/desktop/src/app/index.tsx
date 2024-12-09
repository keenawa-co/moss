import "@/app/i18n";

import { useAtom } from "jotai";
import { Suspense } from "react";
import { useSelector } from "react-redux";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { terminalVisibilityAtom } from "@/atoms/layoutAtom";
import { ContentLayout, LaunchPad, Menu, RootLayout } from "@/components";
import { usePrepareWindow } from "@/hooks/usePrepareWindow";
import { Home, Logs, Settings } from "@/pages";
import { RootState } from "@/store";

import { PageLoader } from "../components/PageLoader";
import { Resizable, ResizablePanel } from "../components/Resizable";

const App = () => {
  const { isPreparing } = usePrepareWindow();

  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);
  const [isTerminalVisible, setIsTerminalVisible] = useAtom(terminalVisibilityAtom);

  if (isPreparing) {
    return <PageLoader />;
  }

  return (
    <RootLayout>
      <Resizable proportionalLayout={false} onVisibleChange={(_index, value) => console.log(_index, value)}>
        <ResizablePanel minSize={100} preferredSize={255} snap visible={isSidebarVisible} className="select-none">
          <LaunchPad />
        </ResizablePanel>
        <ResizablePanel>
          <Resizable
            vertical
            onVisibleChange={(_index, value) => {
              if (_index === 1) setIsTerminalVisible(value);
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
            <ResizablePanel preferredSize={144} snap minSize={100} maxSize={500} visible={isTerminalVisible}>
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
    </RootLayout>
  );
};

export default App;
