# Design

This is a living document and will be updated as our journey unfolds.

## Basic Problem

Package managers solve composition and dependency for prebuilt artefact and attempts to fit the entire solution to the local system based on the repository candidates.

As virtual and alternative candidates become possible, simple graphs become ever more complex. Versioned dependencies, conflicts and priorities are then used to try to locally solve a composition issue.

Lastly, any binary distro ultimately suffers from a fundamental flaw: lack of bootstrap support.

## Our Approach

We will be aiming for a source-first design with full chain invalidation and utilisation of a caching system. While some kind of "package" will exist it will simply be the hash-keyed build of a specific package. Missing package **builds** will be rebuilt and force recomputation of the queue.

We will explicitly target multi-stage bootstrap through automation to ensure that not only can we gain full rebootstrap of our toolchain, but the entire reverse dependency graph is also consistent. Likewise, this will provide a mechanism for breaking circular dependencies like `util-linux` <-> `systemd`.

## Stateless by design

As part of the original Clear Linux team, I helped pioneer the concept of "stateless" in Linux distributions and independently pushed for it for a number of years in Solus, Serpent OS / AerynOS etc. Later this became adopted under the name "hermetic". We will be aiming for the same thing.

## Immutable throughout.

Every build will be performed in specially produced containers using cached artefacts. The aim is to use an evolved version of the approach I created in `boulder` for AerynOS. The cached builds will essentially contain unique files from the `DESTDIR` (installtree) in a random-access friendly format. A local content-addressable store will be referenced in generating an `erofs` "image" on-demand for all build-roots / installs / etc.
