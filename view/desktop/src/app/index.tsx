import "@/app/i18n";

import { Suspense } from "react";
import { useSelector } from "react-redux";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { ContentLayout, LaunchPad, Menu, RootLayout } from "@/components";
import { usePrepareWindow } from "@/hooks/usePrepareWindow";
import { Home, Logs, Settings } from "@/pages";
import { RootState } from "@/store";

import { PageLoader } from "../components/PageLoader";
import { Resizable, ResizablePanel } from "../components/Resizable";
import Provider from "./Provider";

const App = () => {
  const { isPreparing } = usePrepareWindow();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);

  if (isPreparing) {
    return <PageLoader />;
  }

  return (
    <Provider>
      <RootLayout>
        <Resizable proportionalLayout={false}>
          <ResizablePanel minSize={100} preferredSize={255} snap visible={isSidebarVisible} className="select-none">
            <LaunchPad />
          </ResizablePanel>
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
        </Resizable>
      </RootLayout>
    </Provider>
  );
};

export default App;
