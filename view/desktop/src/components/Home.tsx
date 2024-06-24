import { useTranslation } from 'react-i18next'
import { commands } from '../bindings'
import { useState, useEffect } from 'react'

export const Home = () => {
  const { t } = useTranslation(['ns1', 'ns2'])
  const [name, setName] = useState('')

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
  }, [])

  return (
    <main>
      <h1>{t('title')}</h1>
      {name && <p>{t('user', { name: name })}</p>}
      <span>{t('description.part1')}</span>
      <span>{t('description.part1', { ns: 'ns2' })}</span>
    </main>
  )
}
