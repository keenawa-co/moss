import { About, Content, DraggableTopBar, Home, Menu, Properties, RootLayout, Sidebar } from '@/components'
import { Suspense, useState } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './i18n'
import { HomeIcon, Icon, MenuItem, IconTitle } from '../../shared/ui/src'
import StatusBar from './components/StatusBar'

function App() {
  return (
    <>
      <DraggableTopBar />
      <RootLayout>
        <Sidebar className="p-0">
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <HomeIcon className="text-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900" title="Home" />
          </MenuItem>
        </Sidebar>

        <Content className="relative flex flex-col">
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
