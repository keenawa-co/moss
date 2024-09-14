// import React, { useEffect, useRef } from "react";
// import { BoxPanel, DockPanel, Widget } from "@lumino/widgets";

// import "../../node_modules/@lumino/dragdrop/style/index.css";
// import "../../node_modules/@lumino/widgets/style/index.css";
// import "../../node_modules/@lumino/default-theme/style/commandpalette.css";
// import "../../node_modules/@lumino/default-theme/style/datagrid.css";
// import "../../node_modules/@lumino/default-theme/style/dockpanel.css";
// import "../../node_modules/@lumino/default-theme/style/menu.css";
// import "../../node_modules/@lumino/default-theme/style/menubar.css";
// import "../../node_modules/@lumino/default-theme/style/scrollbar.css";
// import "../../node_modules/@lumino/default-theme/style/tabbar.css";

// const LuminoLayoutTest: React.FC = () => {
//   const dockPanelRef = useRef<HTMLDivElement>(null);

//   useEffect(() => {
//     if (dockPanelRef.current) {
//       // Create a new Lumino DockPanel instance
//       const dockPanel = new BoxPanel();
//       dockPanel.id = "lumino-dockpanel";
//       // Attach the DockPanel to the DOM
//       dockPanelRef.current.appendChild(dockPanel.node);

//       // Create a few widgets to add to the DockPanel
//       const widget1 = new Widget();
//       widget1.node.textContent = "Widget 1";
//       widget1.title.label = "First Widget";

//       const widget2 = new Widget();
//       widget2.node.textContent = "Widget 2";
//       widget2.title.label = "Second Widget";

//       // Add widgets to the DockPanel
//       dockPanel.addWidget(widget1);
//       dockPanel.addWidget(widget2);

//       // Ensure the DockPanel resizes with the window
//       window.addEventListener("resize", () => dockPanel.update());
//     }
//   }, []);

//   return (
//     <div
//       ref={dockPanelRef}
//       className={"main"}
//       style={{ height: "100%", width: "100%", position: "relative", paddingTop: 50 }}
//     />
//   );
// };

// export default LuminoLayoutTest;
