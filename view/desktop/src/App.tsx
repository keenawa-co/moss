import { ContentLayout, Home, Logs, Menu, RootLayout, Settings } from "@/components";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { Suspense, useEffect, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { RootState, useAppDispatch } from "./store";
import { setLanguageFromLocalStorage } from "./store/languages/languagesSlice";
import { initializeThemes } from "./store/themes/themesSlice";
import { useSelector } from "react-redux";
import Sidebar from "./components/Sidebar";

function App() {
  const dispatch = useAppDispatch();
  const [sideBarVisible] = useState(true);
  const selectedTheme = useSelector((state: RootState) => state.themes.selected);

  useEffect(() => {
    dispatch(setLanguageFromLocalStorage());
    dispatch(initializeThemes());
  }, []);

  return (
    <>
      {!selectedTheme ? (
        <div className="relative min-h-screen flex bg-storm-800">
          <div className="container max-w-screen-xl mx-auto flex justify-center items-center text-4xl text-white">
            Loading...
          </div>
        </div>
      ) : (
        <RootLayout>
          <Resizable proportionalLayout={false}>
            <ResizablePanel minSize={100} preferredSize={255} snap visible={sideBarVisible}>
              <Sidebar />
            </ResizablePanel>
            <ResizablePanel>
              <ContentLayout className="content relative flex flex-col overflow-auto h-full">
                <Suspense fallback="loading">
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
      )}
    </>
  );
}
export default App;
