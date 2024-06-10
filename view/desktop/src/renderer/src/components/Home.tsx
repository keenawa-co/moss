import { useTranslation } from 'react-i18next'

export const Home = () => {
  const { t } = useTranslation(['ns1', 'ns2'])
  // TODO: Info
  // const { t } = useTranslation('ns2') - if both part1 and part2 are used from 'ns2'
  // const { t } = useTranslation() - or in this way

  return (
    <main>
      <h1 className="dark:text-black">{t('title')}</h1>
      <span className="dark:text-black">{t('description.part1')} </span>
      <span className="dark:text-black">{t('description.part1', { ns: 'ns2' })} </span>
    </main>
  )
}
