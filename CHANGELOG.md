# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0](https://github.com/screwyprof/favkit/compare/v0.4.0...v1.0.0) (2024-12-23)


### ⚠ BREAKING CHANGES

* Desktop and Downloads are now handled as custom locations with full paths instead of special targets with ~ notation.

### Features

* simplify target handling to use full paths ([#12](https://github.com/screwyprof/favkit/issues/12)) ([ac0e996](https://github.com/screwyprof/favkit/commit/ac0e99656b22724001addacc687945ee2471ec7a))

## [0.4.0](https://github.com/screwyprof/favkit/compare/v0.3.0...v0.4.0) (2024-12-22)


### Features

* add special handling for Downloads folder ([8e5817b](https://github.com/screwyprof/favkit/commit/8e5817bda365d04a79130e5887646459ae1190dd))

## [0.3.0](https://github.com/screwyprof/favkit/compare/v0.2.0...v0.3.0) (2024-12-22)


### Features

* handle special locations (Recents & Applications) in Finder sidebar ([#9](https://github.com/screwyprof/favkit/issues/9)) ([101102b](https://github.com/screwyprof/favkit/commit/101102b2462796415a38efd4e92bc4336170a32b))

## [0.2.0](https://github.com/screwyprof/favkit/compare/v0.1.2...v0.2.0) (2024-12-22)


### Features

* improve AirDrop display in Finder sidebar ([#6](https://github.com/screwyprof/favkit/issues/6)) ([767c9aa](https://github.com/screwyprof/favkit/commit/767c9aa6fc9abefcd49d376b56773645f25cf24f))

## [0.1.2](https://github.com/screwyprof/favkit/compare/v0.1.1...v0.1.2) (2024-12-22)


### Bug Fixes

* correct release artifact packaging permissions ([bb9f4e4](https://github.com/screwyprof/favkit/commit/bb9f4e4a1d5e2834d6c5af66c15d233b5a67f7ed))

## [0.1.1](https://github.com/screwyprof/favkit/compare/v0.1.0...v0.1.1) (2024-12-22)


### Bug Fixes

* trigger release with improved artifact packaging ([4634922](https://github.com/screwyprof/favkit/commit/4634922fa3f46176f57096424d2c9deacea20730))

## 0.1.0 (2024-12-21)


### Features

* add git hooks with pre-commit checks ([1a3f935](https://github.com/screwyprof/favkit/commit/1a3f93521195e06fd2edd664451fe70a6d325462))
* add Makefile linting and improve organization ([c08dcfd](https://github.com/screwyprof/favkit/commit/c08dcfdf738bbe0b5f3b8b0531461cb3b8e68169))
* add minimal FinderApi with acceptance test ([486e0a9](https://github.com/screwyprof/favkit/commit/486e0a92a7210fbc14de14f0332f1aac45de1ffe))
* add nix-filter and improve build targets ([3e22dfe](https://github.com/screwyprof/favkit/commit/3e22dfe1681f6a6687fcefa0ec69bd54e9a044fa))
* add security audit to lint target ([0aeb152](https://github.com/screwyprof/favkit/commit/0aeb1526aa6415ad0a6f5eecbcfdb83c9e5773fb))
* **favorites:** add URL resolution for sidebar items ([c5af86d](https://github.com/screwyprof/favkit/commit/c5af86dff69e2d0e1583a40e809135af068f61b0))
* implement display name retrieval for favorites ([6e28a64](https://github.com/screwyprof/favkit/commit/6e28a6498b2e35bcb70009055c8f1a17702393bd))
* introduce anti-corruption layer for favorites ([88725da](https://github.com/screwyprof/favkit/commit/88725dae859ea6c7d6aaf4f75ed3c598b067877a))
* introduce MacOS API abstraction ([31ccccd](https://github.com/screwyprof/favkit/commit/31ccccd0a4925a0774356c71498481c2631f63ad))
* migrate to cargo-nextest ([020da32](https://github.com/screwyprof/favkit/commit/020da3222ddbfbd89e9f08e6dc37e29f61728059))
* **sidebar:** implement Display trait for SidebarItem ([c211697](https://github.com/screwyprof/favkit/commit/c2116979721edeca1224682773f5312731509658))
