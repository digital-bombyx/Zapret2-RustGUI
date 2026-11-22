# Changelog

Все заметные изменения проекта. Формат — [Keep a Changelog](https://keepachangelog.com/ru/1.1.0/), версии — [SemVer](https://semver.org/lang/ru/).

Раздел версии целиком попадает в описание релиза на GitHub, поэтому пишите его для пользователей. Неизданные изменения копите в `[Unreleased]`, а перед тегом `vX.Y.Z` переименуйте его в `[X.Y.Z] - дата`.

## [Unreleased]

## [0.1.1] - 2026-11-22

### Что нового

- **Плоский интерфейс.** Блоки без градиентов и теней, с янтарной полосой слева, в стиле карточек профиля.
- **Меньше памяти в трее.** Закрытие крестиком выгружает окно вместе с WebView2 из памяти, обход продолжает работать. При открытии из трея окно создаётся заново за доли секунды. Запуск с Windows в трей вообще не создаёт окно.
- **Сворачивание экономнее.** Свёрнутый WebView2 приостанавливается и переходит в режим пониженного потребления памяти.
- Новые скриншоты в README, отдельный комплект для английской версии.

### Обновление

Ставьте поверх v0.1.0: пользовательские списки, Game Filter, IPSet и настройки сохраняются. Перед установкой выключите обход и выйдите из приложения через трей.

### What's new

- **Flat UI.** Cards without gradients or shadows, with an amber accent bar, like the profile cards.
- **Less RAM in the tray.** Closing to tray unloads the window together with WebView2; the bypass keeps running. Opening from the tray recreates the window instantly. Starting with Windows creates no window at all.
- **Lighter when minimized.** WebView2 is suspended and switched to a low memory usage target.
- New README screenshots, with a separate set for the English version.

### Upgrading

Install over v0.1.0: user lists, Game Filter, IPSet and settings are kept. Turn the bypass off and quit from the tray before installing.

## [0.1.0] - 2026-11-22

Первый релиз. / First release.

- Включение обхода в один клик: `winws2.exe` процессом без консоли или службой `zapret`, совместимой с `service.bat`.
- Стратегии читаются прямо из `general*.bat`; на главной видны техники обхода выбранной стратегии.
- Тесты стратегий по образцу `utils\test zapret.ps1`: стандартный (HTTP / TLS 1.2 / TLS 1.3 + ping) и DPI «16-20 КБ», с выбором лучшей и историей.
- Game Filter, IPSet, редактор пользовательских списков, управляемый блок в hosts.
- Трей, автозапуск с правами администратора, интерфейс на русском и английском.
- Встроенная сборка zapret-discord-youtube 1.10.4 с PR #18202 (zapret2).

[Unreleased]: https://github.com/digital-bombyx/Zapret2-RustGUI/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/digital-bombyx/Zapret2-RustGUI/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/digital-bombyx/Zapret2-RustGUI/releases/tag/v0.1.0
