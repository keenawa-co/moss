import { ContentLayout, LaunchPad, Menu, RootLayout } from "@/components";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { Suspense } from "react";
import { useSelector } from "react-redux";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { Home, Logs, Settings } from "./components/pages";
import { useInitializeApp } from "./hooks/useInitializeApp";
import { RootState } from "./store";
import { callServiceMethod } from "./main";

const App: React.FC = () => {
  const { isInitializing, initializationError } = useInitializeApp();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);

  callServiceMethod("doSomething");

  if (isInitializing) {
    return (
      <div className="relative flex min-h-screen bg-storm-800">
        <div className="container mx-auto flex max-w-screen-xl items-center justify-center text-4xl text-white">
          Loading...
        </div>
      </div>
    );
  }

  if (initializationError) {
    return (
      <div className="relative flex min-h-screen bg-storm-800">
        <div className="container mx-auto flex max-w-screen-xl flex-col items-center justify-center text-2xl text-red-500">
          <p>Initialization Failed</p>
          <p>{initializationError.message}</p>
        </div>
      </div>
    );
  }

  return (
    <RootLayout>
      <Resizable proportionalLayout={false}>
        <ResizablePanel minSize={100} preferredSize={255} snap visible={isSidebarVisible} className="select-none">
          <LaunchPad />
        </ResizablePanel>
        <ResizablePanel>
          <ContentLayout className="content relative flex h-full flex-col overflow-auto">
            <Suspense fallback={<div className="loading">Loading...</div>}>
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
  );
};

export default App;
