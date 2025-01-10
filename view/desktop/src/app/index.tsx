import "@/app/i18n";

import { AppLayout, RootLayout } from "@/components";
import { usePrepareWindow } from "@/hooks/usePrepareWindow";

import { PageLoader } from "../components/PageLoader";

const App = () => {
  const { isPreparing } = usePrepareWindow();

  if (isPreparing) {
    return <PageLoader />;
  }

  return (
    <RootLayout>
      <AppLayout />
    </RootLayout>
  );
};

export default App;
