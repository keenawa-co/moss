import { useTranslation } from 'react-i18next'

export const About = () => {
  const { t } = useTranslation()

  return (
    <main>
      <h1 className="dark:text-black">{t('about')}</h1>
      <span className="dark:text-black">{t('user', { name: 'Jevgenijs 🦇' })}</span>
    </main>
  )
}
