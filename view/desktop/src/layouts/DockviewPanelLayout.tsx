const DockviewPanelLayout = ({ children }: { children: React.ReactNode }) => {
  return <div className="h-full overflow-auto">{children}</div>;
};

export default DockviewPanelLayout;