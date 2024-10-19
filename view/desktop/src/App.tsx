import { ContentLayout, LaunchPad, Menu, RootLayout } from "@/components";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { Suspense, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { Home, Logs, Settings } from "./components/pages";
import { RootState, useAppDispatch } from "./store";
import { setLanguageFromLocalStorage } from "./store/languages/languagesSlice";
import { initializeThemes } from "./store/themes";

function App() {
  const dispatch = useAppDispatch();

  const [sideBarVisible] = useState(true);
  const isThemeSelected = useSelector((state: RootState) => state.themes.isThemeSelected);

  useEffect(() => {
    dispatch(setLanguageFromLocalStorage());
    dispatch(initializeThemes());
  }, []);

  return (
    <>
      {!isThemeSelected ? (
        <div className="relative flex min-h-screen bg-storm-800">
          <div className="container mx-auto flex max-w-screen-xl items-center justify-center text-4xl text-white">
            Loading...
          </div>
        </div>
      ) : (
        <RootLayout>
          <Resizable proportionalLayout={false}>
            <ResizablePanel minSize={100} preferredSize={255} snap visible={sideBarVisible} className="select-none">
              <LaunchPad />
            </ResizablePanel>
            <ResizablePanel>
              <ContentLayout className="content relative flex h-full flex-col overflow-auto">
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
