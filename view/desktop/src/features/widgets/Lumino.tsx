import React, { useEffect, useRef, useState, useCallback, Suspense } from "react";
import { BoxPanel, DockPanel, Widget } from "@lumino/widgets";
import { Provider, useSelector } from "react-redux";
import { store, useAppDispatch } from "../../app/store";
import { selectWidgets, AppWidget, deleteWidget, activateWidget } from "./widgetsSlice";
import "./Lumino.css";
import * as Pages from "../../pages/index";
import { createRoot } from "react-dom/client";
export type PagesComps = keyof typeof Pages;
/**
 * LuminoWidget allows us to fire custom events to the HTMLElement that is holding all
 * the widgets. This approach handles the plumbing between Lumino and React/Redux
 */
class LuminoWidget extends Widget {
  name: string; // will be displayed in the tab
  closable: boolean; // make disable closing on some widgets if you want
  mainRef: HTMLDivElement; // reference to the element holding the widgets to fire events
  constructor(id: string, name: string, mainRef: HTMLDivElement, closable = true) {
    super({ node: LuminoWidget.createNode(id) });

    this.id = id;
    this.name = name;
    this.mainRef = mainRef;
    this.closable = closable;

    this.setFlag(Widget.Flag.DisallowLayout);
    this.addClass("content");

    this.title.label = name; // this sets the tab name
    this.title.closable = closable;
  }

  static createNode(id: string) {
    const div = document.createElement("div");
    div.setAttribute("id", id);
    return div;
  }

  /**
   * this event is triggered when we click on the tab of a widget
   */
  onActivateRequest(msg: any) {
    // create custom event
    const event = new CustomEvent("lumino:activated", this.getEventDetails());
    // fire custom event to parent element
    this.mainRef?.dispatchEvent(event);
    // continue with normal Widget behaviour
    super.onActivateRequest(msg);
  }

  /**
   * this event is triggered when the user clicks the close button
   */
  onCloseRequest(msg: any) {
    // create custom event
    const event = new CustomEvent("lumino:deleted", this.getEventDetails());
    // fire custom event to parent element
    this.mainRef?.dispatchEvent(event);
    // continue with normal Widget behaviour
    super.onCloseRequest(msg);
  }

  /**
   * creates a LuminoEvent holding name/id to properly handle them in react/redux
   */
  private getEventDetails(): LuminoEvent {
    return {
      detail: {
        id: this.id,
        name: this.name,
        closable: this.closable,
      },
    };
  }
}

/**
 * This is the type of the custom event we use to communicate from lumino to react/redux
 */
export interface LuminoEvent {
  detail: { id: string; name: string; closable: boolean };
}

/**
 * Props of any component that will be rendered inside a LuminoWidget
 */
export interface ReactWidgetProps {
  id: string;
  name: string;
}

/**
 * Type of any component that will be rendered inside a LuminoWidget
 */
export type ReactWidget = React.FC<ReactWidgetProps>;

/**
 * Method to return the component corresponding to the widgettype
 */
const getComponent = (type: PagesComps | PagesComps): ReactWidget => {
  if (Pages[type]) {
    return Pages[type];
  }
  return () => <div>fall back components</div>;
};

/**
 * Initialize Boxpanel and Dockpanel globally once to handle future calls
 */
const main = new BoxPanel({ direction: "left-to-right", spacing: 0 });
const dock = new DockPanel({});

/**
 * This component watches the widgets redux state and draws them
 */
const Lumino: React.FC = () => {
  const [attached, setAttached] = useState(false); // avoid attaching DockPanel and BoxPanel twice
  const mainRef = useRef<HTMLDivElement>(null); // reference for Element holding our Widgets
  const [renderedWidgetIds, setRenderedWidgetIds] = useState<string[]>([]); // tracker of components that have been rendered with LuminoWidget already
  const widgets = useSelector(selectWidgets); // widgetsState
  const dispatch = useAppDispatch();

  /**
   * creates a LuminoWidget and adds it to the DockPanel. Id of widget is added to renderedWidgets
   */
  const addWidget = useCallback((w: AppWidget) => {
    if (mainRef.current === null) return;
    setRenderedWidgetIds((cur) => [...cur, w.id]);
    const lum = new LuminoWidget(w.id, w.tabTitle, mainRef.current, true);
    console.log("dock on addWidget", dock, main);
    dock.addWidget(lum);
  }, []);

  /**
   * watch widgets state and calls addWidget for Each. After addWidget is executed we look
   * for the element in the DOM and use React to render the Component into the widget
   * NOTE: We need to use Provider in order to access the Redux State inside the widgets.
   */
  useEffect(() => {
    if (!attached) return;
    widgets.forEach((w, index) => {
      if (renderedWidgetIds.includes(w.id)) return; // avoid drawing widgets twice
      addWidget(w); // addWidget to DOM
      const el = document.getElementById(w.id); // get DIV
      const Component = getComponent(w.type); // get Component for TYPE
      if (el) {
        createRoot(el).render(
          // draw Component into Lumino DIV
          // <Provider store={store}>
          <Suspense fallback={<div className="w-full">Loading...</div>}>
            <div className="h-full w-full overflow-auto widget-wrapper">
              <Component id={w.id} name={w.tabTitle} />
            </div>
          </Suspense>
          // </Provider>
        );
      }
    });
  }, [widgets, attached, addWidget, renderedWidgetIds]);

  /**
   * This effect initializes the BoxPanel and the Dockpanel and adds event listeners
   * to dispatch proper Redux Actions for our custom events
   */
  useEffect(() => {
    if (mainRef.current === null || attached === true) {
      return;
    }

    main.id = "main";
    main.addClass("main");
    dock.id = "dock";
    window.onresize = () => main.update();
    BoxPanel.setStretch(dock, 1);

    try {
      Widget.attach(main, mainRef.current); // Attach main once
      main.addWidget(dock); // Add dock to main
      setAttached(true);
    } catch (error) {
      console.log();
      console.error("Error attaching widget:", error);
    }

    // Add event listeners for custom events
    mainRef.current.addEventListener("lumino:activated", (e: Event) => {
      const le = e as unknown as LuminoEvent;
      dispatch(activateWidget(le.detail.id));
    });

    mainRef.current.addEventListener("lumino:deleted", (e: Event) => {
      const le = e as unknown as LuminoEvent;
      dispatch(deleteWidget(le.detail.id));
    });
  }, [mainRef, attached, dispatch]);

  useEffect(() => {
    // console.log(widgets);
  }, [widgets]);

  return <div ref={mainRef} className="main h-full w-full" />;
};

export default Lumino;
