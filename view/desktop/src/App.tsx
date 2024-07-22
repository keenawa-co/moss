import { About, Content, DraggableTopBar, Home, Menu, Properties, RootLayout, Sidebar } from '@/components'
import { Suspense } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './i18n'
import { IconHome } from '../../shared/ui/src/IconHome'
import StatusBar from './components/StatusBar'

function App() {
  return (
    <>
      <DraggableTopBar />
      <RootLayout>
        <Sidebar className="p-2 border-2 border-red-500">
          Sidebar
          <IconHome className="text-red-500 w-18 h-18" />
        </Sidebar>

        <Content className="border-2 border-blue-500  relative flex flex-col">
          <Suspense fallback="loading">
            <BrowserRouter>
              <Menu />
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/about" element={<About />} />
              </Routes>
            </BrowserRouter>
          </Suspense>
          <StatusBar
            className="sticky bottom-0 mt-auto"
            branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri"
          />
        </Content>
        <Properties className="p-2 border-2 border-green-500">P</Properties>
      </RootLayout>
    </>
  )
}

export default App
