# Changelog

## [4.0.3](https://github.com/deadnews/imgup/compare/v4.0.2...v4.0.3) - 2026-06-24

### Features

- _(hostings)_ add `filepost`, `imglink`, `kappa`, `pixvid` ([#338](https://github.com/deadnews/imgup/issues/338)) - ([05307ff](https://github.com/deadnews/imgup/commit/05307ff68c702eec05eb8bd93966a7e802dc414d))

### Bug fixes

- improve error handling ([#337](https://github.com/deadnews/imgup/issues/337)) - ([ff2770f](https://github.com/deadnews/imgup/commit/ff2770f05fd39be48d3a114ad7b84f45c293795d))

### Refactor

- simplify - ([7dc66bf](https://github.com/deadnews/imgup/commit/7dc66bf657f024b91cc66d766c447db71ff66832))
- simplify ([#335](https://github.com/deadnews/imgup/issues/335)) - ([94c71cb](https://github.com/deadnews/imgup/commit/94c71cb92099c37e270ab66ad986c976efc8a094))

## [4.0.2](https://github.com/deadnews/imgup/compare/v4.0.1...v4.0.2) - 2026-04-06

### Features

- add `cloudinary`, `gofile`, `imagekit`, `imghippo` ([#333](https://github.com/deadnews/imgup/issues/333)) - ([18aef66](https://github.com/deadnews/imgup/commit/18aef66a76ae74f28f7c80ea2bfa50d650eccbc1))

## [4.0.1](https://github.com/deadnews/imgup/compare/v4.0.0...v4.0.1) - 2026-04-06

### Features

- add `imgbox`, `postimages` ([#332](https://github.com/deadnews/imgup/issues/332)) - ([5aa8b7c](https://github.com/deadnews/imgup/commit/5aa8b7c941d8a5ccda0161de019d267e5fb4616c))

## [4.0.0](https://github.com/deadnews/imgup/compare/v3.0.6...v4.0.0) - 2026-04-03

### Features

- rewrite in rust ([#326](https://github.com/deadnews/imgup/issues/326)) - ([1a5b152](https://github.com/deadnews/imgup/commit/1a5b15208a057129eb9221995951121a94b755d7))

### Chores

- _(git-cliff)_ update config - ([339866e](https://github.com/deadnews/imgup/commit/339866ec8b7e9a13fe2f53ecabf630092413232d))
- _(prek)_ add `zizmor` hook ([#315](https://github.com/deadnews/imgup/issues/315)) - ([e33cdcb](https://github.com/deadnews/imgup/commit/e33cdcb50336f1eedd8e936b4911143ea1ae5973))
- _(vscode)_ update inlay hints setting for python - ([c74ece2](https://github.com/deadnews/imgup/commit/c74ece2254b93f2a17238f4eb61d602e28b731ca))
- replace `mypy` and `pyright` with `ty` - ([b662e75](https://github.com/deadnews/imgup/commit/b662e756cf00694feb43de758be5710fb3f403ed))

## [3.0.6](https://github.com/deadnews/imgup/compare/v3.0.5...v3.0.6) - 2025-07-06

### Features

- migrate from `poetry` to `uv` ([#301](https://github.com/deadnews/imgup/issues/301)) - ([caa2bcc](https://github.com/deadnews/imgup/commit/caa2bcc438691a3603624cf58f0b1a96fca29ae3))

### Chores

- _(config)_ migrate config .renovaterc.json ([#278](https://github.com/deadnews/imgup/issues/278)) - ([e8a3c66](https://github.com/deadnews/imgup/commit/e8a3c66b7f631f201e814c56ae92f176190f5a04))
- _(github)_ reduce usage of `macos` and `windows` runners - ([443e1ef](https://github.com/deadnews/imgup/commit/443e1efe5b56cb487cb1f24ccfe3c0846dfbfd0f))

### Dependencies

- update dependency click to v8.1.8 ([#271](https://github.com/deadnews/imgup/issues/271)) - ([3ad94e0](https://github.com/deadnews/imgup/commit/3ad94e0bd586a352bc433d772afd81048b3a9b7c))
- update dependency pillow to v11.3.0 [security] ([#300](https://github.com/deadnews/imgup/issues/300)) - ([6426760](https://github.com/deadnews/imgup/commit/64267602db9282dbae0a5d3cbf89bd9f9a0ee694))
- update dependency python-dotenv to v1.1.0 ([#286](https://github.com/deadnews/imgup/issues/286)) - ([7fd62b6](https://github.com/deadnews/imgup/commit/7fd62b6a14c212c37b3db39f624a63e7698c6ab1))
- update dependency rich to v14 ([#287](https://github.com/deadnews/imgup/issues/287)) - ([bcafd39](https://github.com/deadnews/imgup/commit/bcafd390dc5337d407c2d4688a16ff4ff1e490d2))

## [3.0.5](https://github.com/deadnews/imgup/compare/v3.0.4...v3.0.5) - 2024-12-15

### Chores

- _(config)_ migrate config .renovaterc.json ([#257](https://github.com/deadnews/imgup/issues/257)) - ([c08f711](https://github.com/deadnews/imgup/commit/c08f711911c91fff61bf65fd602961871b793db4))
- _(github)_ update `build-win` to `python:3.13` ([#270](https://github.com/deadnews/imgup/issues/270)) - ([9ad2853](https://github.com/deadnews/imgup/commit/9ad285390263b1f55e4ebf62e4161e76a7131a25))
- _(renovate)_ group `httpx` and `pytest-httpx` in one PR - ([b3fa5d5](https://github.com/deadnews/imgup/commit/b3fa5d5459acd47c50e46d4cc08e25110b5c0fc0))

### Dependencies

- update httpx and pytest-httpx ([#269](https://github.com/deadnews/imgup/issues/269)) - ([83324f7](https://github.com/deadnews/imgup/commit/83324f7620ff7581a3f50453b4aef4ea723cb07a))
- update dependency loguru to v0.7.3 ([#264](https://github.com/deadnews/imgup/issues/264)) - ([8713c43](https://github.com/deadnews/imgup/commit/8713c430c4ef27ebadc412214a177416164c7050))
- update dependency pillow to v11 ([#252](https://github.com/deadnews/imgup/issues/252)) - ([2c7aeb0](https://github.com/deadnews/imgup/commit/2c7aeb0b7c6a913c8f8efe87ad1e0070f27d30cb))
- update dependency rich to v13.9.4 ([#256](https://github.com/deadnews/imgup/issues/256)) - ([c014852](https://github.com/deadnews/imgup/commit/c014852bf9e01bfe2b26250f8b9885077d7e7451))

## [3.0.4](https://github.com/deadnews/imgup/compare/v3.0.3...v3.0.4) - 2024-10-11

### Bug fixes

- now `pixeldrain` require an `api key` ([#249](https://github.com/deadnews/imgup/issues/249)) - ([4ff3f6d](https://github.com/deadnews/imgup/commit/4ff3f6d86e7f7e96c33d462bc24fa962e3712373))

### Documentation

- _(changelog)_ update `git-cliff` config - ([f7f987d](https://github.com/deadnews/imgup/commit/f7f987dffc23bb783fd842c3dc7ecba80f5cefa3))

### Testing

- fix deprecations ([#248](https://github.com/deadnews/imgup/issues/248)) - ([4958b5f](https://github.com/deadnews/imgup/commit/4958b5f0bd44c325e2d8d0944919edc68eb9e3bf))

### Chores

- _(github)_ add `python:3.13` to tests matrix ([#247](https://github.com/deadnews/imgup/issues/247)) - ([0fbbe48](https://github.com/deadnews/imgup/commit/0fbbe48179740d962f1c301ca8eaa29ed27b8e8a))
- _(github)_ update `build-win` to python `3.12` ([#223](https://github.com/deadnews/imgup/issues/223)) - ([526910a](https://github.com/deadnews/imgup/commit/526910ad8a7e3949e7fdc09ff179affe5a59d698))
- _(github)_ update `aur-deploy` job - ([0738a55](https://github.com/deadnews/imgup/commit/0738a552717bead992ded73eb07d1b57a6ac5e68))
- _(typos)_ ignore short words - ([7b84695](https://github.com/deadnews/imgup/commit/7b846953521c9271a1d1121c30610fc9a19cd1d3))
- remove `httpx` timeout - ([7a33752](https://github.com/deadnews/imgup/commit/7a337526376c0aee683dc67b0b7baed4f54e268a))

### Dependencies

- update dependency httpx to v0.27.2 ([#231](https://github.com/deadnews/imgup/issues/231)) - ([1e75d47](https://github.com/deadnews/imgup/commit/1e75d47af71aaa77b3ce0375bc857f3933d9d9cd))
- update dependency pillow to v10.4.0 ([#218](https://github.com/deadnews/imgup/issues/218)) - ([48a45f4](https://github.com/deadnews/imgup/commit/48a45f4a2eaacf9b7cbfa7ec59dff9001af8ad05))
- update dependency pyperclip to v1.9.0 ([#211](https://github.com/deadnews/imgup/issues/211)) - ([d39dc81](https://github.com/deadnews/imgup/commit/d39dc810cfbdc5babba38487b69a675a2ddd905f))
- update dependency rich to v13.9.2 ([#240](https://github.com/deadnews/imgup/issues/240)) - ([cdd8537](https://github.com/deadnews/imgup/commit/cdd853738a62abf87c72bccaceb0b9af883cfea2))

## [3.0.3](https://github.com/deadnews/imgup/compare/v3.0.2...v3.0.3) - 2024-04-08

### Chores

- _(github)_ update `build-win` job ([#197](https://github.com/deadnews/imgup/issues/197)) - ([d1eedc7](https://github.com/deadnews/imgup/commit/d1eedc72f1d9d3701935568526b34134c2d381d8))
- _(github)_ update aur release job ([#196](https://github.com/deadnews/imgup/issues/196)) - ([4e2e87c](https://github.com/deadnews/imgup/commit/4e2e87c8564dced1c3bd61da1c1274c7643edd1a))
- _(makefile)_ update `release` command ([#198](https://github.com/deadnews/imgup/issues/198)) - ([953207d](https://github.com/deadnews/imgup/commit/953207dfa6f490f05e71e22a42068e9716b4d842))

## [3.0.2](https://github.com/deadnews/imgup/compare/v3.0.1...v3.0.2) - 2024-04-03

### Bug fixes

- update deprecated name for `pillow 10.3.0` compatibility ([#189](https://github.com/deadnews/imgup/issues/189)) - ([c0c5897](https://github.com/deadnews/imgup/commit/c0c5897ad27c22c80ee7e2e7dbe7a6eaf6f3f4b5))

### Documentation

- _(changelog)_ add `git-cliff` ([#186](https://github.com/deadnews/imgup/issues/186)) - ([64b44d4](https://github.com/deadnews/imgup/commit/64b44d4cb1baa36679c6708702dfc63810385e14))
- _(mkdocs)_ add ([#184](https://github.com/deadnews/imgup/issues/184)) - ([cd2fbf0](https://github.com/deadnews/imgup/commit/cd2fbf0cd8de48db713c89dbd43c11d6a9400896))
- _(readme)_ add badges - ([8912d71](https://github.com/deadnews/imgup/commit/8912d71b9a2a60090f072d666901e0b7abcd5144))

### Chores

- update linting tasks in `makefile` and `poe` - ([e01404a](https://github.com/deadnews/imgup/commit/e01404aad59b559f7d148fa3fed520b2e4a78942))

### Dependencies

- update dependency pillow to v10.3.0 ([#190](https://github.com/deadnews/imgup/issues/190)) - ([df49044](https://github.com/deadnews/imgup/commit/df490441833f37ac17777e984015f9af4245c6e8))
- update dependency rich to v13.7.1 ([#179](https://github.com/deadnews/imgup/issues/179)) - ([9191acc](https://github.com/deadnews/imgup/commit/9191acca8ff27f32e16afc4ae38360f73a9644ca))

## [3.0.1](https://github.com/deadnews/imgup/compare/v2.0.1...v3.0.1) - 2024-02-22

### Features

- add logger and error handling ([#175](https://github.com/deadnews/imgup/issues/175)) - ([15678ee](https://github.com/deadnews/imgup/commit/15678ee29bb848663d093407405bb496c85a4759))
- add `anhmoe` image hosting ([#174](https://github.com/deadnews/imgup/issues/174)) - ([c1f401b](https://github.com/deadnews/imgup/commit/c1f401b8f0e9d7dda089a912d8f4cacd03a54864))
- update public accessible objects of that module ([#171](https://github.com/deadnews/imgup/issues/171)) - ([99b81de](https://github.com/deadnews/imgup/commit/99b81de5e9bb7d31a2301908c4de44de17789ba2))

### Bug fixes

- [**breaking**] remove the `-c/-C` shortcut from the `clipboard` cli option ([#177](https://github.com/deadnews/imgup/issues/177)) - ([0aafcce](https://github.com/deadnews/imgup/commit/0aafcce7e63c0e5fdd35d9184b7d7bae185f4a53))

### Documentation

- update docstrings ([#176](https://github.com/deadnews/imgup/issues/176)) - ([1f5b20d](https://github.com/deadnews/imgup/commit/1f5b20dfa0ddf2065efbd21456fcd5a1c1f4b9a0))

### Testing

- update tests ([#122](https://github.com/deadnews/imgup/issues/122)) - ([6226443](https://github.com/deadnews/imgup/commit/622644371147c16b5e872bdd9a06bf523cd749b4))

### Styling

- update `ruff` settings ([#162](https://github.com/deadnews/imgup/issues/162)) - ([ca58de3](https://github.com/deadnews/imgup/commit/ca58de3b98400bf586d06f03b6b55f6d7503a400))

### Chores

- _(pre-commit)_ add `checkmake` hook - ([85c19cb](https://github.com/deadnews/imgup/commit/85c19cbd6b0e22cc7e5b192f62967581887c33a5))
- _(pre-commit)_ add `actionlint` hook - ([8a2ceb1](https://github.com/deadnews/imgup/commit/8a2ceb140ffb94485a029d96f16e98b8de262e54))
- _(pre-commit)_ use `black` mirror - ([90444aa](https://github.com/deadnews/imgup/commit/90444aa8b25c8e8b34f6c2d1db72ba934facceb4))
- build a `windows` executable using the `nuitka` compiler ([#167](https://github.com/deadnews/imgup/issues/167)) - ([a28be07](https://github.com/deadnews/imgup/commit/a28be079833a80cdfae5eb6bbb0d941647c5bc13))
- replace `black` with `ruff` - ([46cf164](https://github.com/deadnews/imgup/commit/46cf1644ee9e6d48b0b96305746da937a2365069))
- add `python 3.12` to tests matrix ([#138](https://github.com/deadnews/imgup/issues/138)) - ([904be1b](https://github.com/deadnews/imgup/commit/904be1b03d5faa6f09bb226efd655e412eaa6408))
- use `environment` for `aur` deploy - ([92df766](https://github.com/deadnews/imgup/commit/92df76614c1860959d34f60d07903b3f258a6835))
- update docstrings ([#132](https://github.com/deadnews/imgup/issues/132)) - ([44fe8e6](https://github.com/deadnews/imgup/commit/44fe8e603682a4efdaccf030bfd68f56e65d55cf))
- specify python `target-version` - ([794a622](https://github.com/deadnews/imgup/commit/794a622befd3d9c9e300057b0c8f088aa375c7b0))
- disable `codeql` on `schedule` - ([25fd100](https://github.com/deadnews/imgup/commit/25fd10032f0b57c129c72bb98b19bbaf92c4ea18))

### Dependencies

- update dependency click to v8.1.7 ([#127](https://github.com/deadnews/imgup/issues/127)) - ([a5da102](https://github.com/deadnews/imgup/commit/a5da1029c3087685464448bed15605ee6fd9d5d0))
- update dependencies ([#158](https://github.com/deadnews/imgup/issues/158)) - ([ac8078d](https://github.com/deadnews/imgup/commit/ac8078d41bfb28e33934e3d81666dbd8bb33078b))
- update dependency httpx to v0.25.2 ([#148](https://github.com/deadnews/imgup/issues/148)) - ([df8ef71](https://github.com/deadnews/imgup/commit/df8ef7137f62f65f136d8d54219a45d4d5465749))
- update dependency pillow to v10.2.0 ([#157](https://github.com/deadnews/imgup/issues/157)) - ([8073735](https://github.com/deadnews/imgup/commit/8073735d0515ff31ca50ec50928337c3efc4f4fe))
- update dependency python-dotenv to v1.0.1 ([#161](https://github.com/deadnews/imgup/issues/161)) - ([f7f57f3](https://github.com/deadnews/imgup/commit/f7f57f3006d81aa1fb42a365fcd97318a80d732b))

## [2.0.1](https://github.com/deadnews/imgup/compare/v2.0.0...v2.0.1) - 2023-07-11

### Features

- call `get_font` only once ([#117](https://github.com/deadnews/imgup/issues/117)) - ([2be7eca](https://github.com/deadnews/imgup/commit/2be7eca2a0a3fb2584be2e2f472e5b13649f9c06))

### Testing

- rename `.env.sample` - ([73c59c5](https://github.com/deadnews/imgup/commit/73c59c50fb1b927b071981ea1065ac14cd335fe0))

### Dependencies

- update dependency pillow to v10 ([#113](https://github.com/deadnews/imgup/issues/113)) - ([6bd957f](https://github.com/deadnews/imgup/commit/6bd957f6ed7f54758c21a6631cf611d17e542efe))

## [2.0.0](https://github.com/deadnews/imgup/compare/v1.1.3...v2.0.0) - 2023-06-24

### Features

- simplify cli - ([c01c6c5](https://github.com/deadnews/imgup/commit/c01c6c53db06999009bbbfda7009f81ee2d4af07))
- use `asyncio` ([#106](https://github.com/deadnews/imgup/issues/106)) - ([d6966de](https://github.com/deadnews/imgup/commit/d6966deac152c974a9d0e73c3674859877e76dcc))
- rename `get_env` func - ([b5fea88](https://github.com/deadnews/imgup/commit/b5fea88d7cb929340411b25afc9d7cbbb7ebfd70))

### Documentation

- fix `workflow` name - ([1dd9115](https://github.com/deadnews/imgup/commit/1dd91159420a93b877ab3881b5164952756ff9c4))

### Testing

- use `pragma: no cover` ([#110](https://github.com/deadnews/imgup/issues/110)) - ([c47567a](https://github.com/deadnews/imgup/commit/c47567a3eccf2f17bfae1d252854b39c336d5f44))

### Chores

- _(pre-commit)_ add `typos` hook - ([892b6ce](https://github.com/deadnews/imgup/commit/892b6cebf0c2cd047467cf6220f4d3d91266ece4))
- _(renovate)_ use shared config - ([9fe50ad](https://github.com/deadnews/imgup/commit/9fe50ad0c2bf4fdcc69d33bb719144a0ab683dbe))
- use `digest pinning` - ([d9bc707](https://github.com/deadnews/imgup/commit/d9bc707990478082e22c0bcc5f10d3aa2575f6f9))
- rename `deps-review` - ([aafd1fe](https://github.com/deadnews/imgup/commit/aafd1fe60484a4ae3f6b7cd91cf68ecc8fc23c1a))
- rename poetry `group` - ([98852e1](https://github.com/deadnews/imgup/commit/98852e134d1810ed3d12cbccc7ede5ceae6c78a4))
- update `workflows` ([#98](https://github.com/deadnews/imgup/issues/98)) - ([342e77c](https://github.com/deadnews/imgup/commit/342e77cba5d31ce778eea876ba44485f12062f6b))

### Dependencies

- update dependency requests to v2.31.0 ([#95](https://github.com/deadnews/imgup/issues/95)) - ([3b302dc](https://github.com/deadnews/imgup/commit/3b302dc67172fc0e3cf62d82d37ad09204cf8e86))

## [1.1.3](https://github.com/deadnews/imgup/compare/v1.1.1...v1.1.3) - 2023-05-08

### Build

- update `PKGBUILD` ([#86](https://github.com/deadnews/imgup/issues/86)) - ([864d257](https://github.com/deadnews/imgup/commit/864d257202732bdd31af81fe0c705cae5e00f3d2))

### Chores

- fix deploy to `aur` - ([571b763](https://github.com/deadnews/imgup/commit/571b7639a01b227f82748ffb6468af88d2c9b89d))

## [1.1.1](https://github.com/deadnews/imgup/commits/v1.1.1) - 2023-05-07

### Features

- dev pr ([#77](https://github.com/deadnews/imgup/issues/77)) - ([9b3e7a6](https://github.com/deadnews/imgup/commit/9b3e7a68e21d343e03634eff7e1ac55b8448d276))

### Bug fixes

- _(build)_ enable `poetry-dynamic-versioning` - ([5007861](https://github.com/deadnews/imgup/commit/50078619083700a8f8fc0765ac05c272f08cf3a3))

### Dependencies

- update dependency requests to v2.30.0 - ([6d3932b](https://github.com/deadnews/imgup/commit/6d3932b28a81d9ee0db85cc1ad8e54c05376a658))

<!-- generated by git-cliff -->
