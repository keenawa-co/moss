// TODO: Info
// Trans component can also be used for translation
import { Button } from '@/components'
import { useEffect, useState } from 'react'
import { Trans, useTranslation } from 'react-i18next'
import { NavLink } from 'react-router-dom'
import { LANGUAGES } from '../constants/index'

const isActive = ({ isActive }: any) => `link ${isActive ? 'active' : ''}`

export const Menu = () => {
  // Translation
  const { i18n, t } = useTranslation()

  const onChangeLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const lang_code = e.target.value
    i18n.changeLanguage(lang_code)
  }

  // Dark Mode

  const [darkMode, setDarkMode] = useState<boolean | undefined>(undefined)

  const switchMode = () => {
    setDarkMode(!darkMode)
  }

  useEffect(() => {
    if (darkMode) {
      localStorage.setItem('darkMode', 'true')
      window.document.documentElement.classList.add('dark')
    } else if (darkMode === false) {
      localStorage.setItem('darkMode', 'false')
      window.document.documentElement.classList.remove('dark')
    } else {
      setDarkMode(localStorage.getItem('darkMode') === 'true')
    }
  }, [darkMode])

  const themes = ['black', 'orange', 'purple', 'green', 'blue']
  const [theme, setTheme] = useState<string>(themes[0])

  const switchTheme = (newTheme: string) => {
    setTheme(newTheme)
  }

  useEffect(() => {
    if (darkMode) {
      localStorage.setItem('darkMode', 'true')
      window.document.documentElement.classList.add('dark')
    } else if (darkMode === false) {
      localStorage.setItem('darkMode', 'false')
      window.document.documentElement.classList.remove('dark')
    } else {
      setDarkMode(localStorage.getItem('darkMode') === 'true')
    }
  }, [theme])

  const onChangeLTheme = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newTheme = e.target.value
    setTheme(newTheme)
  }

  console.log('---------------->' + theme)

  return (
    <nav className={`theme-${theme}`}>
      <p>
        <Trans i18nKey="title" className="text-primary">
          Welcome to react using <code>react-i18next</code> fully type-safe
        </Trans>
      </p>
      <div>
        <NavLink className={isActive + ' bg-primary'} to="/">
          {t('home')}
        </NavLink>
        <NavLink className={isActive + ' bg-secondary'} to="/about">
          {t('about')}
        </NavLink>
      </div>

      <select className="bg-green-500" defaultValue={i18n.language} onChange={onChangeLang}>
        {LANGUAGES.map(({ code, label }) => (
          <option key={code} value={code}>
            {label}
          </option>
        ))}
      </select>

      <div>
        <Button
          border="none"
          color="pink"
          height="50px"
          onClick={switchMode}
          radius="50%"
          width="50px"
          children="Mode"
        />
      </div>

      <select className="bg-green-500" defaultValue={themes[0]} onChange={onChangeLTheme}>
        {themes.map((t) => (
          <option key={t} value={t}>
            {t}
          </option>
        ))}
      </select>
    </nav>
  )
}
