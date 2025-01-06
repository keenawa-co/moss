import "@/app/i18n";

import { RootLayout } from "@/components";
import { usePrepareWindow } from "@/hooks/usePrepareWindow";
import { AppLayout } from "@/layouts/AppLayout";

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
