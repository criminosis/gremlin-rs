# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.7](https://github.com/criminosis/gremlin-rs/compare/gremlin-client-v0.8.6...gremlin-client-v0.8.7) - 2024-02-14

### Fixed
- fix from tungstenite error ([#198](https://github.com/criminosis/gremlin-rs/pull/198))
- fix compilation issue
- fixes [#158](https://github.com/criminosis/gremlin-rs/pull/158) ([#179](https://github.com/criminosis/gremlin-rs/pull/179))
- fix channel close on drop
- fix fmt
- fix uuid update
- fix withSideEffect implementation ([#160](https://github.com/criminosis/gremlin-rs/pull/160))
- fixed example
- *(Async)* Add supperingnore cert option(tokio).
- *(Async)* Add supperingnore cert option(async-std). [https://github.com/wolf4ood/gremlin-rs/issues/86]
- fixed test for choose
- fixed cargo fmt
- fixed fmt issue
- fixed fmt issue

### Other
- release ([#193](https://github.com/criminosis/gremlin-rs/pull/193))
- update tungstenite
- update webpki
- Update rustls requirement from 0.19 to 0.20 in /gremlin-client ([#144](https://github.com/criminosis/gremlin-rs/pull/144))
- Update base64 requirement from 0.13.1 to 0.21.4 in /gremlin-client ([#195](https://github.com/criminosis/gremlin-rs/pull/195))
- bumped to client 0.8.5
- updated changelog
- bumped to 0.8.4
- Exposed websocket max_message_size  through ConnectionOptions ([#189](https://github.com/criminosis/gremlin-rs/pull/189))
- Add .id() step to the traversal builder ([#188](https://github.com/criminosis/gremlin-rs/pull/188))
- Support Property/VertexProperty in TryFrom<GValue> ([#185](https://github.com/criminosis/gremlin-rs/pull/185))
- add sideEffect step ([#183](https://github.com/criminosis/gremlin-rs/pull/183))
- Add .e traversal step ([#182](https://github.com/criminosis/gremlin-rs/pull/182))
- updates changelog
- Removes GraphSON v1 ([#178](https://github.com/criminosis/gremlin-rs/pull/178))
- Edge labels should rely on Labels type, not string ([#175](https://github.com/criminosis/gremlin-rs/pull/175))
- bumped to 0.8.3
- Remove Websocket & Time crates ([#172](https://github.com/criminosis/gremlin-rs/pull/172))
- *(Readme)* fix https://github.com/wolf4ood/gremlin-rs/issues/168
- bumped to 0.8.2
- update changelog
- Update async-tungstenite requirement in /gremlin-client ([#163](https://github.com/criminosis/gremlin-rs/pull/163))
- Remove noisy rustc warnings ([#154](https://github.com/criminosis/gremlin-rs/pull/154))
- Update base64 requirement from 0.12.1 to 0.13.1 in /gremlin-client ([#162](https://github.com/criminosis/gremlin-rs/pull/162))
- Add elementMap step ([#148](https://github.com/criminosis/gremlin-rs/pull/148))
- bumped to 0.8.1
- Collection Type Conversions ([#147](https://github.com/criminosis/gremlin-rs/pull/147))
- Expose AsyncTerminator as gremlin::client::aio:AsyncTerminator ([#139](https://github.com/criminosis/gremlin-rs/pull/139))
- Bug fixes.
- Check pool connections.
- Make test sessioned_graph mutable.
- Formatted code.
- Added close_session to tests.
- Added ability to close sessions properly.
- Implemented ToGValue for Vec<GValue>
- bumped to 0.8.0
- Restoring #[test] lost from rebase
- Removed parallel TryFrom<GValue> for f32 and f64
- Map null property to an empty list/set
- Formatting
- Removed unused import
- Drop defunct method
- Enabled set support for other types
- Enabled list support for other types
- Added SET and LIST cardinality support for String
- Fixed / Added Option support for: String, i32, i64, f32, f64, uuid,
- Formatting
- Fixed precision of g:Date values to milliseconds
- Updated changelog
- Bumped to 0.7.1
- Changed SessionedClient to new type pattern.
- Fixed test case.
- Interim progress.
- updated changelog
- Bumped to 0.7.0
- fmt fix
- bumped tokio to v1
- Expose `AsyncTerminator` publicly
- Bumped to 0.6.2
- fmt
- expose generate_message fn as public
- add until, repeat & emit
- property_many step on TraversalBuilder
- tryfrom gvalue for bool
- Updated readme
- updated changelog
- Bumped to 0.6.1
- added Send + Sync to conversion trait
- Updated changelog
- Closes [#97](https://github.com/criminosis/gremlin-rs/pull/97).
- Added derive docs
- added derive example
- clippy fix
- Merge pull request [#94](https://github.com/criminosis/gremlin-rs/pull/94) from bertdemiranda/add_other_and_both_traversal_steps
- Changes made by rustfmt
- Add both and other traversals
- Updated changelog
- Merge branch 'master' into async-tls-fix
- added derive for converting from gremlin_client::Map -> struct
- gremlin derive for handy conversion
- Bumped to 0.5.2
- updated changelog
- updated changelog
- Bumped to 0.5.1
- cargo fmt
- disable shutdown on drop
- Replace block_on with spawn on connection drop
- .property_with_id() step not needed
- use unique property_with_id step instead of property accepting both &str & T
- Merge branch 'multiple-remotes-feature' of github.com:jdeepee/gremlin-rs into multiple-remotes-feature
- added possibility for .property step to take either &str or T
- added constant to anon traversal source
- added barrier and optional step
- Added some traversal methods to make it easy to change remotes post traversal creation
- Updated changelog
- Bumped to 0.5.0
- cleanup where step
- cleanup until step
- cleanup to step
- cleanup select step
- cleanup repeat step
- cleanup or step
- cleanup not step
- cleanup match step
- cleanup loop step
- cleanup local step
- clean up from step
- cleanup coalesce step
- cleanup by step
- cleanup has step
- Cargo fmt
- Changed Into impl to From
- restored async-tls 0.6.0
- Bumped async-tls to 0.7.0
- Bumped base64 to 0.12.1
- Updated changelog
- Corrected merge.
- Merge from master.
- Updated changelog
- Bumped to 0.4.0
- Updated docs
- Updated general readme
- Updated changelog
- Fixed tests
- Fixed fmt
- Added 1 more async example
- Fmt fix
- Removed warning + Refactored test
- Added tokio impl for connection
- Started tokio support impl: added usage of futures channels
- Fmt fix
- Tests refactor
- Fixed fmt
- Updated Changelog
- Removed println
- Printlns removed.
- Added support for GraphSONv1
- Fixed warning
- Bumped to 0.3.2
- updated changelog
- added identity, range and cap to graph traversal
- added out_v, is, or,where_ and cap steps to anon traversal
- Merge branch 'master' of https://github.com/wolf4ood/gremlin-rs
- Added additional coalesce test
- First impl of coalesce https://github.com/wolf4ood/gremlin-rs/issues/67
- Added round in profile test
- Bumped async-tungstenite to v0.4.0
- Updated changelog
- cargo fmt
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/69
- Fixed fmt
- Fixed warning into_iter()
- Updated Changelog
- fmt
- added aggregate step
- added select to anon graph traversal
- Added more text p methods
- Bumped to 3.0.1
- Updated changelog
- Eliminated duplicate version enum and renamed version as serializer.
- Clean up unused imports.
- Added GraphSON v2 support.
- bumped to mobc 0.5.3
- added ability to send bool and vec of strings as labels
- added support for sending vertices property cardinality on two new traversal functions
- updated changelog
- with async method rename to with_remote_async
- updated .gitignore
- Bumped async-tungstenite to 0.3.1
- updated to uuid 0.8.1
- updated to mobc 0.4.1
- fmt fix
- bumped to async-tungstenite 0.2.1
- Update websocket requirement from 0.23 to 0.24 in /gremlin-client
- added out_e to anon traversal
- Fixed fmt with 1.40
- Bumped to 0.3.0
- Updated readme
- updated example in lib.rs
- Updated description of the driver in lib.rs
- Added explicit features in Cargo.toml for tests and examples
- added traversal example async
- Removed unwrap/expect
- Fixed warning + async_std feature
- cargo fmt fix
- Progress of async impl. It still need some polish before merge
- fmt
- added integration tests for new traversal methods
- sample step should be i32 not i64
- added local method on graph traversal
- implements loops method on anonymous traversal and graph traversal
- implement sample step
- adds simple_path method
- implement basic until step
- implement repeat method on graphtraversal
- fmt
- makes FromGValue public in crate
- Implements a with_side_effect method on GraphTraversalSource
- Fixed fmt
- Merge pull request [#46](https://github.com/criminosis/gremlin-rs/pull/46) from jdeepee/property_has_many
- fmt
- Implements Pop enum and .v() within traversal
- allows multiple GID's to be used
- Updated Changelog
- Bumped to 0.2.2
- implements from for string on gid and into_iter on list
- implements basic text_p with "containing" functionality.
- Implemented https://github.com/wolf4ood/gremlin-rs/issues/41
- Fixed fmt
- Update base64 requirement from 0.10.1 to 0.11.0 in /gremlin-client
- Merge branch 'master' into map_step
- add basic project step
- Renamed HasStepKey::STRING -> HasStepKey::Str
- fmt
- Added ability to use T attributes for has key
- Renamed Label::String_ -> Label::Str
- Merge pull request [#32](https://github.com/criminosis/gremlin-rs/pull/32) from jdeepee/bool_value_map_label
- added impls to bool gvalue
- fmt
- added from uuid trait for GID
- added integration test for utils function
- added basic unwrap map util to help with unpacking of vertices values
- fmt
- added bool integration test for value map
- added bool label - useful for valueMap(true) where a user might want to return vertices properties AND id
- Fix fmt issue
- fmt
- added basic integration tests for or step
- added basic or trait
- Updated Changelog
- added dyn to trait object in examples
- I have generated a self-signed SSL certificate for testing.
- support SSL
- Bumped to 0.2.1
- Added [#1](https://github.com/criminosis/gremlin-rs/pull/1) to changelog
- Implemented Plain SASL Authentication https://github.com/wolf4ood/gremlin-rs/issues/1
- Update websocket requirement from 0.22 to 0.23 in /gremlin-client
- Updated changelog
- Bumped to 0.2.0
- Fixes fmt issue on traversal example
- added example with traversal condition in where
- Added terminator API iter
- added group_as for group step with sideeffect key
- added group_count_as for group count step with sideeffect key
- Added traversal example
- Added a more complex traversal example and traversal doc test
- Added Next and Has Next step
- Fixed fmt
- restored (()) for empty params
- Added drop step
- Fixed compilation issue
- Implemented conversion up to 10 size for array of params
- Refactor traversal by introducing TraversalBuilder
- Revert "Added api for boxing the traversal. Useful for match step with vec! of traversal"
- Added api for boxing the traversal. Useful for match step with vec! of traversal
- added not step in AnonymousTraversalSource
- Added support for match step https://github.com/wolf4ood/gremlin-rs/issues/22
- Fixed clippy hint
- Replaced (()) with ({}) for empty params
- Fixed fmt issue
- Exposed SyncTerminator
- Started Big refactor with Terminator step as Trait
- added support for valueMap step https://github.com/wolf4ood/gremlin-rs/issues/22
- Supported by with two parameters
- Added support for order type on by step
- Added order step https://github.com/wolf4ood/gremlin-rs/issues/22
- added has to anonymous traversal
- added not step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added where step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added is step
- Fixed failing tests
- Introduced IntoPredicate for every T that impl ToGValue for conversion to Predicate
- added support for predicate within
- added min step
- added mean step
- added max step
- Added sum step
- moved Scope into process::traversal module
- Added neq/gt/gte/lt/lte to P
- Added unfold step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added dedup step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added values step to __ traversal
- Added integration test for limit step
- Fixed fmt issue
- Added Limit Step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added Path Step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added Fold Step https://github.com/wolf4ood/gremlin-rs/issues/22
- Fixed clippy issues
- Implemented select step https://github.com/wolf4ood/gremlin-rs/issues/22
- added out step in anonymous traversal
- added first support for anonymouse traversal with __ , applied also in by step
- refactor traversal into traversal module
- added group step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added support for T type and by with T https://github.com/wolf4ood/gremlin-rs/issues/22
- Fist support of by step https://github.com/wolf4ood/gremlin-rs/issues/22
- Added group count step https://github.com/wolf4ood/gremlin-rs/issues/22
- Implemented Count Step https://github.com/wolf4ood/gremlin-rs/issues/22
- added hasNot step https://github.com/wolf4ood/gremlin-rs/issues/22
- added has with 1 params for checking if a property exists
- implemented has step with (label,key,value)
- HasStep refactor in order to use a tuple for step parameters
- Added Values Step https://github.com/wolf4ood/gremlin-rs/issues/22
- Exported GraphTraversalSource in crate::process
- updated changelog
- added step propertyMap https://github.com/wolf4ood/gremlin-rs/issues/19
- Added properties step + GProperty type that wraps Property and VertexProperty https://github.com/wolf4ood/gremlin-rs/issues/19
- aded label step
- updated changelog
- Added out_v/in_v step https://github.com/wolf4ood/gremlin-rs/issues/21
- Added in_e/out_e step https://github.com/wolf4ood/gremlin-rs/issues/21
- Implemented in step wiht in_ function https://github.com/wolf4ood/gremlin-rs/issues/21
- Updated changelog
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/18
- Updated changelog
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/20
- updated changelog
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/17
- Fixed fmt issue
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/16
- added deterministic condition in test_edge_query
- Updated changelog
- Added another traversal in the example
- Added traversal example
- Fixed clippy warnings
- Fixed fmt issue
- Refactor g.v and g.e for handling ids paramenters
- Added out step https://github.com/wolf4ood/gremlin-rs/issues/12
- improved ergonomics of hasLabel step
- adde #[allow(dead_code)] on commons some test functions
- Supported has step + integration tests
- Added integration test for hasLabel step
- Added g.v/e with id traversal tests https://github.com/wolf4ood/gremlin-rs/issues/12
- First simple traversal working g.v() https://github.com/wolf4ood/gremlin-rs/issues/12
- Fixed fmt issue
- Added has + has_label step and started P(predicate) struct
- Started GLV implementation
- updated changelog
- Added support for nested metric https://github.com/wolf4ood/gremlin-rs/issues/14
- Updated changelog
- Bumped to 0.1.2
- Updated vertex example with Vertex APIs
- Updated edge example with usage of Edge APIs
- added api GID::get for getting the inner value with cast
- Added API Edge#property
- Fixed fmt issue
- Removed API Set::new and reduced visibility of List::new
- Reduced visibility of Vertex/Edge::new since are not public apis
- Added doc for Map type
- Added API iter to List/Set
- Added more field types in integration test. Added UUID/DateTime params
- test_complex_vertex_creation_with_properties refactor with propertyMap
- Updated changelog
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/13
- Updated Changelog
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/11
- Updated Changelog
- Fixed fmt
- Implemented https://github.com/wolf4ood/gremlin-rs/issues/10
- Changed args serialization in Request
- Added Map struct instead of alias for HashMap<String,GValue>
- Support Map with T (token) as keys
- Support g:T (token) type
- deserializer refactor. extracted remove behaviour
- small deserializer macro refactor
- Fixed fmt issues
- Bumped to 0.1.1
- Added badges on Cargo.toml
- added reference to IO docs for GraphSON v3
- added integration tests to coverage
- updated changelog
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/3
- Fixed fmt issue
- updated uuid dep to 0.7.2
- Updated changelog
- Fixes https://github.com/wolf4ood/gremlin-rs/issues/4
- Updated changelog
- Merge pull request [#5](https://github.com/criminosis/gremlin-rs/pull/5) from rafaelcaricio/fix-typo
- Fix typo in macro definition
- Added description in Cargo.toml
- Updated metadata in Cargo.toml
- Added more examples
- Refactor from borrow to get
- Added API GValue::borrow for borrowing the owned value instead of taking it with take
- Implemented FromGValue for Property
- Implemented FromGValue for Path
- moved docker-compose to the root
- Azure CI first attempt
- first commit

### Removed
- removed panic warning in tests
- removed custom impl for taking params. Replaced with From impl
- removed example
- removed unnecessary parentheses
- removed commented function in traversal
- removed ToPredicate conversion traits in favour of Into<Predicate> for ergonomic usage
- removed dbg! macros
- removed dbg!
- removed assert on intermediate explain

## [0.8.6](https://github.com/wolf4ood/gremlin-rs/compare/gremlin-client-v0.8.5...gremlin-client-v0.8.6) - 2023-10-19

### Fixed
- fix from tungstenite error ([#198](https://github.com/wolf4ood/gremlin-rs/pull/198))
- fix compilation issue

### Other
- update tungstenite
- update webpki
- Update rustls requirement from 0.19 to 0.20 in /gremlin-client ([#144](https://github.com/wolf4ood/gremlin-rs/pull/144))
- Update base64 requirement from 0.13.1 to 0.21.4 in /gremlin-client ([#195](https://github.com/wolf4ood/gremlin-rs/pull/195))

### Added

### Fixed

### Changed


## [0.8.4] - 2023-05-20

### Added

- [#188](https://github.com/wolf4ood/gremlin-rs/pull/188)
- [#189](https://github.com/wolf4ood/gremlin-rs/pull/189)
- [#182](https://github.com/wolf4ood/gremlin-rs/pull/182)
- [#185](https://github.com/wolf4ood/gremlin-rs/pull/185) 

### Fixed

- Add Labels type in `add_e` [#174](https://github.com/wolf4ood/gremlin-rs/issues/174)

### Changed

- Removed `GraphSON` v1 [#177](https://github.com/wolf4ood/gremlin-rs/issues/177)

## [0.8.3] - 2023-02-06

### Added

### Fixed

- Remove Websocket & Time crates #172 

## [0.8.2] - 2021-05-09

### Added

### Fixed

- Fix connection not closing properly

## [0.8.0] - 2021-05-09

### Added

- [129](https://github.com/wolf4ood/gremlin-rs/pull/129) Added Option support for: String, i32, i64, f32, f64, uuid, date, and bool
- [132](https://github.com/wolf4ood/gremlin-rs/pull/131) Added SET and LIST cardinality support 

### Fixed

### Changed

- [#128](https://github.com/wolf4ood/gremlin-rs/issues/128) Fixed Date serialization precision

## [0.7.1] - 2021-03-03

### Added

- [#116](https://github.com/wolf4ood/gremlin-rs/pull/116) Added support for Session
### Fixed

## [0.7.0] - 2021-02-05

### Added

- [#122](https://github.com/wolf4ood/gremlin-rs/issues/122) Exposed AsyncTerminator
- Updated to Tokio v1
### Fixed

## [0.6.2] - 2020-11-16

### Added

- [#109](https://github.com/wolf4ood/gremlin-rs/pull/109) Added repeat, until, emit steps
- [#102](https://github.com/wolf4ood/gremlin-rs/pull/102) Added property many 

### Fixed

## [0.6.1] - 2020-09-7

### Added

### Fixed

- [#97](https://github.com/wolf4ood/gremlin-rs/issues/97) Fixed issue on boolean deserialization

## [0.6.0] - 2020-07-03

### Added

- [#50](https://github.com/wolf4ood/gremlin-rs/issues/50) First impl of derive from GResult/Map

### Fixed

- [#86](https://github.com/wolf4ood/gremlin-rs/issues/86) Fixed option accept_invalid_certs with async

## [0.5.1] - 2020-06-05

### Added

- [#82](https://github.com/wolf4ood/gremlin-rs/pull/82) Added .project(), .constant() & .barrier() and more.

### Fixed

## [0.5.0] - 2020-05-11

### Added

- [#77](https://github.com/wolf4ood/gremlin-rs/pull/77) Added Iter and IntoIter impl.

### Fixed

## [0.4.0] - 2020-04-18

### Added

- [#74](https://github.com/wolf4ood/gremlin-rs/pull/74) Added support for GraphSONv1
- [#75](https://github.com/wolf4ood/gremlin-rs/issues/75) Added support for Tokio Runtime

### Fixed

## [0.3.2] - 2020-03-22

### Added

- [#67](https://github.com/wolf4ood/gremlin-rs/issues/67) Implemented coalesce 
- [#66](https://github.com/wolf4ood/gremlin-rs/pull/66)  Added anonymous steps (add_v,property) and traversal steps (choose,value)

### Fixed

- [#69](https://github.com/wolf4ood/gremlin-rs/issues/69) Fixed issue with pong messages.

## [0.3.1] - 2020-02-10

### Added

- [#62](https://github.com/wolf4ood/gremlin-rs/issues/62) Added support for GraphSONv2


### Fixed

## [0.3.0] - 2020-01-06

### Added

- [#15](https://github.com/wolf4ood/gremlin-rs/issues/15) Async support
- [#51](https://github.com/wolf4ood/gremlin-rs/pull/51)  Repeat, until, simplePath, sample, loops and local
- [#47](https://github.com/wolf4ood/gremlin-rs/pull/47) Implements Pop enum for .select() and .v() 
- [#48](https://github.com/wolf4ood/gremlin-rs/pull/48) Implements basic with_side_effect
- [#55](https://github.com/wolf4ood/gremlin-rs/pull/55) Added out_e

### Fixed


## [0.2.2] - 2019-11-06

### Added

- [#41](https://github.com/wolf4ood/gremlin-rs/issues/8) Added traversal input for From/To step
- [#31](https://github.com/wolf4ood/gremlin-rs/issues/1) Implemented TextP

### Fixed

## [0.2.1] - 2019-09-13

### Added

- [#8](https://github.com/wolf4ood/gremlin-rs/issues/8) SSL Support
- [#1](https://github.com/wolf4ood/gremlin-rs/issues/1) Implemented SASL Authentication

### Fixed


## [0.2.0] - 2019-06-14

### Added
- [#12](https://github.com/wolf4ood/gremlin-rs/issues/12) GLV support (Base impl)
- [#16](https://github.com/wolf4ood/gremlin-rs/issues/16) Implemented addV Step
- [#17](https://github.com/wolf4ood/gremlin-rs/issues/17) Implemented property Step
- [#20](https://github.com/wolf4ood/gremlin-rs/issues/20) Implemented as Step
- [#18](https://github.com/wolf4ood/gremlin-rs/issues/18) AddEdge Step
- [#21](https://github.com/wolf4ood/gremlin-rs/issues/21) Implemented Remaining Vertex/Edge Step
- [#19](https://github.com/wolf4ood/gremlin-rs/issues/19) properties + propertyMap Step

### Fixed

- [#14](https://github.com/wolf4ood/gremlin-rs/issues/14) Fixed support for nested metrics

## [0.1.2] - 2019-04-04

### Added

- [#11](https://github.com/wolf4ood/gremlin-rs/issues/11) Support for V and E as keys in `Map`.
- [#2](https://github.com/wolf4ood/gremlin-rs/issues/10) Added support for other types as keys in `Map`.

### Changed

- [#13](https://github.com/wolf4ood/gremlin-rs/issues/13) Refactor List/Set in their own types.

## [0.1.1] - 2019-03-27

### Added

- [#2](https://github.com/wolf4ood/gremlin-rs/issues/2) Implemented alias support.

### Fixed

- [#4](https://github.com/wolf4ood/gremlin-rs/issues/4) Fixed traversal metrics eg. `g.V().profile()`
- [#3](https://github.com/wolf4ood/gremlin-rs/issues/3) Fixed traversal exxplanation eg. `g.V().explain()`

## [0.1.0] - 2019-03-18

### Added
- First release

