import { Content, RootLayout, Sidebar } from "@/components"

function App() {
  return (
    <RootLayout>
      <Sidebar className="p-2 border-4 border-red-500">Sidebar</Sidebar>
      <Content className="border-4 border-blue-500">Content</Content>
    </RootLayout>
  )
}

export default App
