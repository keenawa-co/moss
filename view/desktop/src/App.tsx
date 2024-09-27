import { ContentLayout, Menu, RootLayout } from "@/components";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { Suspense, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { Home, Logs, Settings } from "./components/pages";
import { RootState, useAppDispatch } from "./store";
import { setLanguageFromLocalStorage } from "./store/languages/languagesSlice";
import { initializeThemes } from "./store/themes";
import DraggableAccordion from "./components/DraggableAccordion";
import { setAccordion, setDefaultSizes } from "./store/accordion/accordionSlice";
import { swapByIndex } from "./store/accordion/accordionHelpers";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import SidebarHeader from "./components/SidebarHeader";
import * as DesktopComponents from "./components";

type DesktopComponentKeys = keyof typeof DesktopComponents;
type OmittedComponents = Omit<
  Record<DesktopComponentKeys, any>,
  "RootLayout" | "SidebarLayout" | "ContentLayout" | "PropertiesLayout"
>;
type DesktopComponentsOmitted = keyof OmittedComponents;

const getDesktopComponentByName = (name: DesktopComponentsOmitted) => {
  if (!DesktopComponents[name]) return <div>{name}</div>;

  const Tag = DesktopComponents[name];
  return <Tag />;
};

function App() {
  const dispatch = useAppDispatch();
  const [sideBarVisible] = useState(true);
  const selectedTheme = useSelector((state: RootState) => state.themes.selected);
  const accordion = useSelector((state: RootState) => state.accordion.accordion);
  const defaultSizes = useRef(useSelector((state: RootState) => state.accordion.defaultSizes));

  useEffect(() => {
    dispatch(setLanguageFromLocalStorage());
    dispatch(initializeThemes());
  }, []);

  useEffect(() => {
    return monitorForElements({
      onDrop({ source, location }) {
        const destination = location.current.dropTargets[0];
        if (!destination) return;

        const destinationLocation = destination.data.location as number;
        const sourceLocation = source.data.location as number;

        if (
          destinationLocation === undefined ||
          sourceLocation === undefined ||
          destinationLocation === sourceLocation
        ) {
          return;
        }

        const updatedArray = swapByIndex(accordion, sourceLocation, destinationLocation);

        dispatch(setAccordion(updatedArray));
      },
    });
  }, [accordion]);

  const handleAccordionClick = (index: number) => {
    const updatedAccordion = accordion.map((accordion, i) =>
      i === index ? { ...accordion, isOpen: !accordion.isOpen } : accordion
    );

    dispatch(setAccordion(updatedAccordion));
  };

  return (
    <>
      {!selectedTheme ? (
        <div className="relative flex min-h-screen bg-storm-800">
          <div className="container mx-auto flex max-w-screen-xl items-center justify-center text-4xl text-white">
            Loading...
          </div>
        </div>
      ) : (
        <RootLayout>
          <Resizable proportionalLayout={false}>
            <ResizablePanel minSize={100} preferredSize={255} snap visible={sideBarVisible} className="select-none">
              <SidebarHeader title="launchpad" />
              <Resizable
                proportionalLayout={false}
                vertical
                defaultSizes={defaultSizes.current}
                onDragEnd={(e) => dispatch(setDefaultSizes(e))}
                className="pb-[35px]"
              >
                {accordion.map((accordion, index) => (
                  <ResizablePanel key={index} minSize={accordion.isOpen ? 100 : 35}>
                    <DraggableAccordion
                      key={accordion.title}
                      {...accordion}
                      location={index}
                      handleClick={() => handleAccordionClick(index)}
                    >
                      {getDesktopComponentByName(accordion.content as DesktopComponentsOmitted)}
                    </DraggableAccordion>
                  </ResizablePanel>
                ))}
              </Resizable>
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
