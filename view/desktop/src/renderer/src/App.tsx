import { About, Content, DraggableTopBar, Home, Menu, RootLayout, Sidebar } from '@/components'
import { Suspense, useEffect, useState, createContext } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './i18n'
import React from 'react';

function App() {
  
  return (
    <>
      <DraggableTopBar />
      <RootLayout>
        <Sidebar className="p-2 border-4 border-red-500">Sidebar</Sidebar>
        <Content className="border-4 border-blue-500">
          <Suspense fallback="loading">
            <BrowserRouter>
              <Menu />
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/about" element={<About />} />
              </Routes>
            </BrowserRouter>
          </Suspense>
        </Content>
      </RootLayout>
    </>
  )
}

export default App
