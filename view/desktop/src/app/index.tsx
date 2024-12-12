import "@/app/i18n";

import { RootLayout } from "@/components";
import { usePrepareWindow } from "@/hooks/usePrepareWindow";
import { AppLayout } from "@/layouts/AppLayout";

import { PageLoader } from "../components/PageLoader";
import Provider from "./Provider";

const App = () => {
  const { isPreparing } = usePrepareWindow();

  if (isPreparing) {
    return <PageLoader />;
  }

  return (
    <Provider>
      <RootLayout>
        <AppLayout />
      </RootLayout>
    </Provider>
  );
};

export default App;
