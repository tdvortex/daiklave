# daiklave
An Exalted 3e character sheet app and dice roller, written entirely in Rust.

## Status

```daiklave``` is **extremely** under construction at this point and should not be used. Everything is subject to dramatic and irreversible overhauls, refactorings, and breaking changes. If you need a character sheet app right now, use [Lot-Casting Atemi](https://www.lotcastingatemi.com/).

## Goals

* *Support custom content*. Daiklave should allow users to create their own resources with minimal effort. 
* *Support correctness*. It should be difficult or impossible to build an invalid character using Daiklave.
* *Support online play over Discord*. Daiklave should make it easy to play an Exalted 3e game through a Discord server (guild), with no additional support from external VTTs.
* *Support mobile-first design*. Daiklave's design should assume smartphone support, while also supporting desktop browsers.

It is explicitly **not** the goal of the project to provide full programmatic support for all Charms. Exalted breaks its own rules constantly and creatively. Modeling all of these variants in code would be both infeasible, and also also restrictive, as it would prevent new custom homebrew. 

Support for material will be limited to **only** official, published content for Exalted 3e. Kickstarter manuscripts will not be supported until the full public release of the PDF.

## Structure

* ```daiklave-core```: the main API of the application, with the primary interfaces of ```Character``` (for reading and modifying an existing sheet), ```CharacterBuilder``` (for assembling a new character from component parts), and ```GuidedCharacterBuilder``` (for an interactive character creation walkthrough).
* ```daiklave-postgres```: An interface, written using ```sqlx```, for saving and loading characters into a Postgres SQL database.
* ```daiklave-axum```: A webserver, written using ```axum```, to enable a REST API for Discord OIDC authentication, ```daiklave-postgres``` content storage, and HTTP-based requests to the Discord API.
* ```daiklave-yew```: a GUI written using the ```yew``` library, using Tailwind as a CSS library.

Deployment is intended to be through Google Cloud, using Docker containers on Cloud Run.

## Roadmap

### Core Rulebook

- [ ] Add Character and CharacterBuilder for Solars and mortal PCs
- [ ] Add GuidedCharacterBuilder for Solars and mortals
- [ ] Add Postgres CRUD functions for Players, Campaigns, Characters, and all book resources (Charms, Martial Arts, Sorcery, etc.)
- [ ] Add Axum server routes for CRUD functions
- [ ] Add Axum server routes for Discord OIDC
- [ ] Add Yew auth pathway
- [ ] Add Yew renderer for character (read-only)
- [ ] Add Yew character mutations (for existing character)
- [ ] Add Yew character guided builders
- [ ] Add Yew fetching from Axum server routes
- [ ] Add "hello world"-type Discord HTTP route in Axum
- [ ] Add "hello world" route HTTP request route in Yew
- [ ] Dockerize application

### Dice rolling

- [ ] Create basic RollMacro -> RollRequest -> RollResult pathway in ```daiklave-core```
- [ ] Add private-mode dice roller to Yew
- [ ] Add public-mode bot channel dice roller route to Axum
- [ ] Add public-mode dice roller interface to Yew
- [ ] Extend RollMacro, RollRequest, and RollResult to handle dice tricks

### Quick Characters

- [ ] Add QuickCharacter and QuickCharacterBuilder to ```daiklave-core```
- [ ] Add Postgres CRUD functions for QCs
- [ ] Add Axum server routes for QC CRUD
- [ ] Add Yew renderer for QCs

### Lunars
- [ ] Add Lunar configurations to Character, CharacterBuilder, and GuidedCharacterBuilder
- [ ] Update dice roller
- [ ] Update Postgres CRUD functions
- [ ] Update Yew renderer/editer

### Dragon-Blooded
- [ ] Add DB configurations to Character, CharacterBuilder, and GuidedCharacterBuilder
- [ ] Update dice roller
- [ ] Update Postgres CRUD functions
- [ ] Update Yew renderer/editer

### Eventually?

- [ ] Exigents, Sidereals, etc.
- [ ] *Essence* support
- [ ] Progessive Web App (PWA) qualification
- [ ] Image support (profiles pics)
- [ ] Caching using redis