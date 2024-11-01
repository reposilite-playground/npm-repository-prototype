# npm-repository-prototype

This is a prototype project for Reposilite to see what is needed and how long it will 
take to add npm package support. This prototype allow packages publishing, installing them, 
and let users log in with a username and password. The login details are used as a bearer token 
in later requests.
## Storage

All npm packages are kept in folders inside a main folder called `packages`. 
For packages that use a scope, the folders start with `@`, and each folder has its own projects. 
Here is how the storage looks:

- **Packages:** `/packages/<package-name>`
- **Scoped Packages:** `/packages/@<scope>/<package-name>`

Each package has a `metadata.json` file. This file has information about 
the package versions and helps manage multiple versions. In addition to `metadata.json`,
each project has files named `<package-name>-<package-version>.tgz` 
that contain compressed archives of the packages. When a new version is published,
the system checks for an existing `metadata.json` file, loads the current versions 
into a list, and then adds the new version from the request. Finally, the updated list 
is saved back into the `metadata.json` file.