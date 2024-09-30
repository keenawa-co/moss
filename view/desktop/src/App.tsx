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
import Accordion from "./components/DraggableAccordion";
import { setAccordion, setDefaultSizes } from "./store/accordion/accordionSlice";
import SidebarHeader from "./components/SidebarHeader";
import * as DesktopComponents from "./components";
import { DndContext, closestCenter, KeyboardSensor, PointerSensor, useSensor, useSensors } from "@dnd-kit/core";
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { swapByIndex } from "./store/accordion/accordionHelpers";

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
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleDragEnd = (event) => {
    const { active, over } = event;

    if (active.id !== over.id) {
      const oldIndex = accordion.findIndex((a) => a.id === active.id);
      const newIndex = accordion.findIndex((a) => a.id === over.id);

      // console.log({
      //   "active.id":  active.id,
      //   "over.id":  over.id,
      //   "oldIndex":  oldIndex,
      //   "newIndex": newIndex
      // })

      const updated = swapByIndex(accordion, oldIndex, newIndex);
      dispatch(setAccordion(updated));
    }
  };

  useEffect(() => {
    dispatch(setLanguageFromLocalStorage());
    dispatch(initializeThemes());
  }, []);

  const toggleAccordion = (index: number) => {
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
              <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
                <Resizable
                  proportionalLayout={false}
                  vertical
                  defaultSizes={defaultSizes.current}
                  onDragEnd={(e) => dispatch(setDefaultSizes(e))}
                  className="pb-[35px]"
                >
                  <SortableContext items={accordion} strategy={verticalListSortingStrategy}>
                    {accordion.map((accordion, index) => (
                      <ResizablePanel key={accordion.id} minSize={accordion.isOpen ? 100 : 35}>
                        <Accordion key={accordion.id} {...accordion} handleClick={() => toggleAccordion(index)}>
                          {getDesktopComponentByName(accordion.content as DesktopComponentsOmitted)}
                        </Accordion>
                      </ResizablePanel>
                    ))}
                  </SortableContext>
                </Resizable>
              </DndContext>
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
