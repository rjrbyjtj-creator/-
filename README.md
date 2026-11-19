# Madar Robo Desktop

A lightweight Windows desktop shell for the online Madar Robo experience.

## Current architecture

- Tauri 2 desktop shell for Windows.
- Local Persian startup screen with the Madar M mark and square loader.
- Branded M application icon generated from the SVG source during the build.
- Login and chatbot remain hosted by the official Madar GitHub Pages site.
- Authentication and chatbot APIs continue to use the existing Madar services.
- The WebView allows top-level navigation only to the local app and the Madar GitHub Pages host.
- No credentials, API keys, or user data are bundled here.

## Windows build

GitHub Actions builds both NSIS EXE and MSI installers automatically on pushes to `main`, and on version tags such as `v0.1.0`.

1. Open the repository Actions tab.
2. Select Build Windows installer.
3. Open the latest successful run.
4. Download `madar-robo-windows-installers` under Artifacts.

A version tag also creates a GitHub Release with the installers attached. Manual runs are available through **Run workflow**.

## Website integration

The main Madar-F login page has a desktop-only routing branch:

- Regular users who enter through `?desktop=1` are sent directly to the online Robo chatbot after login.
- Existing valid regular-user sessions are also sent directly to Robo.
- Admin accounts retain the existing admin-dashboard route.
- Normal browser login behaviour remains unchanged when `desktop=1` is absent.

## Session behaviour

The app uses the website's WebView profile. The site's existing **Remember me** option remains authoritative: remembered sessions use `localStorage`; unremembered sessions use `sessionStorage` and should be treated as temporary. Do not assume an unremembered session survives an application restart.

## Security

- No CORS bypass or disabled web security.
- No admin credentials or tokens are stored in the desktop app.
- Intended for regular student accounts, not the admin dashboard.


## In-app desktop updates

- A persistent update shortcut is injected only into pages served by the official Madar GitHub Pages origin.
- The shortcut opens a local, branded updater window.
- The updater checks the latest GitHub Release and verifies signed update packages using Tauri's updater plugin.
- Windows update bundles and signatures are attached to tagged GitHub Releases when signing is configured.
- Configure the repository Actions secrets `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`, and place the matching public key in `src-tauri/tauri.conf.json` before publishing signed updates.
- Never commit or share the private signing key. Keep a secure backup: losing it prevents future updates for installations that trust the corresponding public key.
