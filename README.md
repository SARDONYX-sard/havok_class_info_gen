# Tauri Next.js Template(GUI & CLI)

## Template Features

- Frontend
  - Formatter settings
  - Linter settings
  - Localization support
  - Open log button
  - Test environment

- CI(release, build test)

- backend
  - Logger(Rotate files, Change command)

- Crates separation(Core, Cli and GUI)

## For Developer

Development GUI

```shell
npm i # npm install
npm run build # build with release mode
```

See scripts in package.json for other commands.

## Sample GUI images

![Home](https://github.com/SARDONYX-sard/tauri-nextjs-template/assets/68905624/b5053376-cbbb-48af-95fa-40df79b4b3f6)
![Settings](https://github.com/SARDONYX-sard/tauri-nextjs-template/assets/68905624/f1f79d30-a1a5-4b70-ab2a-42ba7c8778f9)

## Replace items

It is recommended that you rename the place marked `Template ~`.

Example

- Cargo.toml
- `.github/workflows/release-cli.yaml`(row200) `App Cli`
- `.github/workflows/release-gui.yaml` `Template App GUI``
- package.json
- `src-tauri/tauri.conf.json`
- Rename app_core, app_cli(then rename Cargo.toml too)

## How to release?

```shell
# git tag <tag version> -m <message> -s <- when use signed commit
git tag "0.1.0" -m "new release"
git push --tags # Then fire release event
```

## License

- Unlicense: You are free to decide the license of this template. We recommend
  MIT OR Apache-2.0.
