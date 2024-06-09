// TODO: Info
// Trans component can also be used for translation
import { useTranslation, Trans } from 'react-i18next'
import { NavLink } from 'react-router-dom'
import { LANGUAGES } from '../constants/index'

const isActive = ({ isActive }: any) => `link ${isActive ? 'active' : ''}`

export const Menu = () => {
  const { i18n, t } = useTranslation()

  const onChangeLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const lang_code = e.target.value
    i18n.changeLanguage(lang_code)
  }

  return (
    <nav>
      <p>
        <Trans i18nKey="title">
          Welcome to react using <code>react-i18next</code> fully type-safe
        </Trans>
      </p>
      <div>
        <NavLink className={isActive + ' bg-blue-400'} to="/">
          {t('home')}
        </NavLink>
        <NavLink className={isActive + ' bg-red-400'} to="/about">
          {t('about')}
        </NavLink>
      </div>

      <select className="text-black" defaultValue={i18n.language} onChange={onChangeLang}>
        {LANGUAGES.map(({ code, label }) => (
          <option key={code} value={code}>
            {label}
          </option>
        ))}
      </select>
    </nav>
  )
}
