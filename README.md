# Madar Robo Desktop

A lightweight Windows desktop shell for the online Madar Robo experience.

## Current architecture

- Tauri 2 desktop shell.
- A small local startup screen with the Madar M mark and square loader.
- Login and chatbot remain hosted by GitHub Pages.
- Authentication and chatbot APIs continue to use the existing Madar services.
- No credentials, API keys, or user data are bundled here.

## Windows build

GitHub Actions builds both NSIS EXE and MSI installers.

1. Open the repository Actions tab.
2. Select Build Windows installer.
3. Click Run workflow on the main branch.
4. After a successful run, open it and download madar-robo-windows-installers under Artifacts.

Pushing a tag such as v0.1.0 also creates a GitHub Release with installers attached.

## Required website integration

The current login page redirects regular users to the normal dashboard. The desktop entry URL includes the query parameter desktop=1, reserved for a narrowly scoped desktop-only routing branch. That branch still needs to be added to the main Madar-F login page so that successful login and session restoration send regular users to the online chatbot. Without that small website-side change, the app will open the normal dashboard after login.

The normal browser flow must remain unchanged when desktop=1 is absent.

## Session behaviour

The app uses the WebView website profile. The site's existing Remember me option remains authoritative: remembered sessions use localStorage; unremembered sessions use sessionStorage and should be treated as temporary. Do not assume an unremembered session survives an application restart.

## Security

- No CORS bypass or disabled web security.
- No admin credentials or tokens are stored in the desktop app.
- Intended for regular student accounts, not the admin dashboard.
