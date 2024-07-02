import { useTranslation } from 'react-i18next'
import { commands, CreateProjectInput } from '../bindings'
import React, { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

export const Home: React.FC = () => {
  const { t } = useTranslation(['ns1', 'ns2'])
  const [name, setName] = useState('')
  const [data, setData] = useState<number | null>(null)

  useEffect(() => {
    const fetchName = async () => {
      try {
        const response = await commands.greet('g10z3r')
        setName(response)
      } catch (error) {
        console.error('Failed to fetch greeting:', error)
      }
    }

    fetchName()

    const unlisten = listen<number>('data-stream', (event) => {
      setData(event.payload)
    })

    return () => {
      unlisten.then((f) => f())
    }
  }, [])

  const handleCreateProject = async () => {
    console.log('Project Test')
    try {
      const input: CreateProjectInput = {
        source: '/Users/g10z3r/bar',
        repository: null
      }
      await commands.createProject(input)
      console.log('Project created successfully')
    } catch (error) {
      console.error('Failed to create project:', error)
    }
  }

  return (
    <main>
      <h1>{t('title')}</h1>
      {name && <p>{t('user', { name })}</p>}
      <span>{t('description.part1')}</span>
      <span>{t('description.part1', { ns: 'ns2' })}</span>
      {data !== null && <p>Received data: {data}</p>}
      <button className="bg-red-500" onClick={handleCreateProject}>
        Create Project
      </button>
    </main>
  )
}
