import * as FlexLayout from "flexlayout-react";
import "flexlayout-react/style/light.css";
import * as Pages from "../pages/index";
import { MutableRefObject, RefObject, Suspense, useEffect, useRef, useState } from "react";
export type PagesComps = keyof typeof Pages;

export function usePrevious<T>(state: T): T | undefined {
  const ref = useRef<T>();

  useEffect(() => {
    ref.current = state;
  });

  return ref.current;
}

// function deepCompare<T>(obj1: T, obj2: T): boolean {
//   // If objects are not the same type, return false
//   if (typeof obj1 !== typeof obj2) {
//     return false;
//   }
//   // If objects are both null or undefined, return true
//   if (obj1 === null && obj2 === null) {
//     return true;
//   }
//   // If objects are both primitive types, compare them directly
//   if (typeof obj1 !== "object") {
//     return obj1 === obj2;
//   }
//   // If objects are arrays, compare their elements recursively
//   if (Array.isArray(obj1) && Array.isArray(obj2)) {
//     if (obj1.length !== obj2.length) {
//       return false;
//     }
//     for (let i = 0; i < obj1.length; i++) {
//       if (!deepCompare(obj1[i], obj2[i])) {
//         return false;
//       }
//     }
//     return true;
//   }
//   // If objects are both objects, compare their properties recursively
//   const keys1 = Object.keys(obj1);
//   const keys2 = Object.keys(obj2);
//   if (keys1.length !== keys2.length) {
//     return false;
//   }
//   for (let key of keys1) {
//     if (!obj2.hasOwnProperty(key) || !deepCompare(obj1[key], obj2[key])) {
//       return false;
//     }
//   }
//   return true;
// }

const jsonLayout: FlexLayout.IJsonModel = {
  global: {
    tabEnableRename: false,
    tabEnableClose: false,
    tabEnableRenderOnDemand: true,
  },
  borders: [],
  layout: {
    type: "row",
    weight: 100,

    children: [
      {
        type: "tabset",
        weight: 100,
        children: [
          {
            type: "tab",
            name: "Home",
            component: "HomePage",
          },
          {
            type: "tab",
            name: "Settings",
            component: "SettingsPage",
          },
          {
            type: "tab",
            name: "Logs",
            component: "LogsPage",
          },
        ],
      },
    ],
  },
};

const layoutModel = FlexLayout.Model.fromJson(jsonLayout);

const FlexLayoutTest = () => {
  const layoutRef = useRef<FlexLayout.Layout>(null);
  const [layoutRevision, setLayoutRevision] = useState(0);
  const prevLayoutRevision = usePrevious(layoutRevision);

  console.log("layoutRef", layoutRef);

  const factory = (node: FlexLayout.TabNode) => {
    const component = node.getComponent() as PagesComps;

    // if (layoutRef.current?.revision !== layoutRevision) {
    //   setLayoutRevision(layoutRef.current?.revision);
    // }

    // console.log(deepCompare(layoutRef.current, prevLayoutRef?.current));
    console.log(layoutRef.current?.revision);

    if (Pages[component]) {
      const PageComponent = Pages[component];
      return (
        <Suspense fallback={<div>Loading...</div>}>
          <PageComponent />
        </Suspense>
      );
    }

    return <div>fall back components</div>;
  };

  // Custom tab rendering
  const onRenderTab = (node: FlexLayout.TabNode, renderValues: FlexLayout.ITabRenderValues) => {
    renderValues.buttons = [
      <button
        key="close"
        className="custom-close-button hover:bg-red-300"
        onClick={() => node.getModel().doAction(FlexLayout.Actions.deleteTab(node.getId()))}
      >
        ❌
      </button>,
    ];

    renderValues.content = (
      <div className="custom-tab">
        <span className="custom-tab-label">{renderValues.content}</span>
      </div>
    );

    renderValues.leading = null;
  };

  return (
    <FlexLayout.Layout ref={layoutRef} model={layoutModel} factory={factory} realtimeResize onRenderTab={onRenderTab} />
  );
};

export default FlexLayoutTest;
