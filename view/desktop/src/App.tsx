import { ContentLayout, LaunchPad, Menu } from "@/components";
import "@/i18n";
import { Suspense } from "react";
import { useSelector } from "react-redux";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { Home, Logs, Settings } from "./pages";
import { useInitializeApp } from "./hooks/useInitializeApp";
import { RootState } from "./store";
import { callServiceMethod } from "./main";
import { PageLoader } from "./components/PageLoader";
import RootLayout from "./components/app/RootLayout";
import Provider from "./components/app/Provider";

const App = () => {
  const { isInitializing, initializationError } = useInitializeApp();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);

  callServiceMethod("doSomething");

  if (isInitializing) {
    return <PageLoader />;
  }

  if (initializationError) {
    return (
      <div className="relative flex min-h-screen bg-storm-800">
        <div className="mx-auto flex max-w-screen-xl flex-col items-center justify-center text-2xl text-red-500">
          <p>Initialization Failed</p>
          <p>{initializationError.message}</p>
        </div>
      </div>
    );
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
