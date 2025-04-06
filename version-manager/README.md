# Command-Line Help for `version`

This document contains the help content for the `version` command-line program.

**Command Overview:**

* [`version`↴](#version)
* [`version major`↴](#version-major)
* [`version major get`↴](#version-major-get)
* [`version major set`↴](#version-major-set)
* [`version major set +`↴](#version-major-set-+)
* [`version major set -`↴](#version-major-set--)
* [`version major set up`↴](#version-major-set-up)
* [`version major set down`↴](#version-major-set-down)
* [`version major reset`↴](#version-major-reset)
* [`version minor`↴](#version-minor)
* [`version minor get`↴](#version-minor-get)
* [`version minor set`↴](#version-minor-set)
* [`version minor set +`↴](#version-minor-set-+)
* [`version minor set -`↴](#version-minor-set--)
* [`version minor set up`↴](#version-minor-set-up)
* [`version minor set down`↴](#version-minor-set-down)
* [`version minor reset`↴](#version-minor-reset)
* [`version patch`↴](#version-patch)
* [`version patch get`↴](#version-patch-get)
* [`version patch set`↴](#version-patch-set)
* [`version patch set +`↴](#version-patch-set-+)
* [`version patch set -`↴](#version-patch-set--)
* [`version patch set up`↴](#version-patch-set-up)
* [`version patch set down`↴](#version-patch-set-down)
* [`version patch reset`↴](#version-patch-reset)
* [`version alpha`↴](#version-alpha)
* [`version alpha get`↴](#version-alpha-get)
* [`version alpha set`↴](#version-alpha-set)
* [`version alpha set +`↴](#version-alpha-set-+)
* [`version alpha set -`↴](#version-alpha-set--)
* [`version alpha set up`↴](#version-alpha-set-up)
* [`version alpha set down`↴](#version-alpha-set-down)
* [`version alpha rm`↴](#version-alpha-rm)
* [`version alpha reset`↴](#version-alpha-reset)
* [`version beta`↴](#version-beta)
* [`version beta get`↴](#version-beta-get)
* [`version beta set`↴](#version-beta-set)
* [`version beta set +`↴](#version-beta-set-+)
* [`version beta set -`↴](#version-beta-set--)
* [`version beta set up`↴](#version-beta-set-up)
* [`version beta set down`↴](#version-beta-set-down)
* [`version beta rm`↴](#version-beta-rm)
* [`version beta reset`↴](#version-beta-reset)
* [`version rc`↴](#version-rc)
* [`version rc get`↴](#version-rc-get)
* [`version rc set`↴](#version-rc-set)
* [`version rc set +`↴](#version-rc-set-+)
* [`version rc set -`↴](#version-rc-set--)
* [`version rc set up`↴](#version-rc-set-up)
* [`version rc set down`↴](#version-rc-set-down)
* [`version rc rm`↴](#version-rc-rm)
* [`version rc reset`↴](#version-rc-reset)
* [`version build`↴](#version-build)
* [`version build get`↴](#version-build-get)
* [`version build set`↴](#version-build-set)
* [`version build rm`↴](#version-build-rm)
* [`version get`↴](#version-get)
* [`version set`↴](#version-set)
* [`version version`↴](#version-version)
* [`version revision`↴](#version-revision)
* [`version file`↴](#version-file)
* [`version file track`↴](#version-file-track)
* [`version file rm`↴](#version-file-rm)
* [`version file update`↴](#version-file-update)
* [`version file update-all`↴](#version-file-update-all)
* [`version file list`↴](#version-file-list)
* [`version package`↴](#version-package)
* [`version package major`↴](#version-package-major)
* [`version package major get`↴](#version-package-major-get)
* [`version package major set`↴](#version-package-major-set)
* [`version package major set +`↴](#version-package-major-set-+)
* [`version package major set -`↴](#version-package-major-set--)
* [`version package major set up`↴](#version-package-major-set-up)
* [`version package major set down`↴](#version-package-major-set-down)
* [`version package major reset`↴](#version-package-major-reset)
* [`version package minor`↴](#version-package-minor)
* [`version package minor get`↴](#version-package-minor-get)
* [`version package minor set`↴](#version-package-minor-set)
* [`version package minor set +`↴](#version-package-minor-set-+)
* [`version package minor set -`↴](#version-package-minor-set--)
* [`version package minor set up`↴](#version-package-minor-set-up)
* [`version package minor set down`↴](#version-package-minor-set-down)
* [`version package minor reset`↴](#version-package-minor-reset)
* [`version package patch`↴](#version-package-patch)
* [`version package patch get`↴](#version-package-patch-get)
* [`version package patch set`↴](#version-package-patch-set)
* [`version package patch set +`↴](#version-package-patch-set-+)
* [`version package patch set -`↴](#version-package-patch-set--)
* [`version package patch set up`↴](#version-package-patch-set-up)
* [`version package patch set down`↴](#version-package-patch-set-down)
* [`version package patch reset`↴](#version-package-patch-reset)
* [`version package alpha`↴](#version-package-alpha)
* [`version package alpha get`↴](#version-package-alpha-get)
* [`version package alpha set`↴](#version-package-alpha-set)
* [`version package alpha set +`↴](#version-package-alpha-set-+)
* [`version package alpha set -`↴](#version-package-alpha-set--)
* [`version package alpha set up`↴](#version-package-alpha-set-up)
* [`version package alpha set down`↴](#version-package-alpha-set-down)
* [`version package alpha rm`↴](#version-package-alpha-rm)
* [`version package alpha reset`↴](#version-package-alpha-reset)
* [`version package beta`↴](#version-package-beta)
* [`version package beta get`↴](#version-package-beta-get)
* [`version package beta set`↴](#version-package-beta-set)
* [`version package beta set +`↴](#version-package-beta-set-+)
* [`version package beta set -`↴](#version-package-beta-set--)
* [`version package beta set up`↴](#version-package-beta-set-up)
* [`version package beta set down`↴](#version-package-beta-set-down)
* [`version package beta rm`↴](#version-package-beta-rm)
* [`version package beta reset`↴](#version-package-beta-reset)
* [`version package rc`↴](#version-package-rc)
* [`version package rc get`↴](#version-package-rc-get)
* [`version package rc set`↴](#version-package-rc-set)
* [`version package rc set +`↴](#version-package-rc-set-+)
* [`version package rc set -`↴](#version-package-rc-set--)
* [`version package rc set up`↴](#version-package-rc-set-up)
* [`version package rc set down`↴](#version-package-rc-set-down)
* [`version package rc rm`↴](#version-package-rc-rm)
* [`version package rc reset`↴](#version-package-rc-reset)
* [`version package build`↴](#version-package-build)
* [`version package build get`↴](#version-package-build-get)
* [`version package build set`↴](#version-package-build-set)
* [`version package build rm`↴](#version-package-build-rm)
* [`version package get`↴](#version-package-get)
* [`version package set`↴](#version-package-set)
* [`version package version`↴](#version-package-version)
* [`version package revision`↴](#version-package-revision)
* [`version package file`↴](#version-package-file)
* [`version package file track`↴](#version-package-file-track)
* [`version package file rm`↴](#version-package-file-rm)
* [`version package file update`↴](#version-package-file-update)
* [`version package file update-all`↴](#version-package-file-update-all)
* [`version package file list`↴](#version-package-file-list)
* [`version package rm`↴](#version-package-rm)
* [`version package list`↴](#version-package-list)

## `version`

A tool for managing the version of a project

**Usage:** `version [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `major` — Change the major version number
* `minor` — Change the minor version number
* `patch` — Change the patch version number
* `alpha` — Change the alpha identifier
* `beta` — Change the beta identifier
* `rc` — Change the release candidate identifier
* `build` — Change the build identifier
* `get` — Get the current version number as a full SemVer string
* `set` — Set the version number to a specific version
* `version` — Get just the version number as a string with no revision or build identifiers
* `revision` — Get just the revision number as a string with no build identifiers
* `file` — Track and update the version number in a file
* `package` — Track and update the version number in a file

###### **Options:**

* `--generator <GENERATOR>` — Generate shell completions

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `version major`

Change the major version number

**Usage:** `version major <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `reset` — Reset the subversions



## `version major get`

Print the current version

**Usage:** `version major get`



## `version major set`

Set the version number

**Usage:** `version major set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version major set +`

Increment the version number by 1

**Usage:** `version major set +`



## `version major set -`

Decrement the version number by 1

**Usage:** `version major set -`



## `version major set up`

Increment the version number by 1

**Usage:** `version major set up`



## `version major set down`

Decrement the version number by 1

**Usage:** `version major set down`



## `version major reset`

Reset the subversions

**Usage:** `version major reset`



## `version minor`

Change the minor version number

**Usage:** `version minor <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `reset` — Reset the subversions



## `version minor get`

Print the current version

**Usage:** `version minor get`



## `version minor set`

Set the version number

**Usage:** `version minor set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version minor set +`

Increment the version number by 1

**Usage:** `version minor set +`



## `version minor set -`

Decrement the version number by 1

**Usage:** `version minor set -`



## `version minor set up`

Increment the version number by 1

**Usage:** `version minor set up`



## `version minor set down`

Decrement the version number by 1

**Usage:** `version minor set down`



## `version minor reset`

Reset the subversions

**Usage:** `version minor reset`



## `version patch`

Change the patch version number

**Usage:** `version patch <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `reset` — Reset the subversions



## `version patch get`

Print the current version

**Usage:** `version patch get`



## `version patch set`

Set the version number

**Usage:** `version patch set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version patch set +`

Increment the version number by 1

**Usage:** `version patch set +`



## `version patch set -`

Decrement the version number by 1

**Usage:** `version patch set -`



## `version patch set up`

Increment the version number by 1

**Usage:** `version patch set up`



## `version patch set down`

Decrement the version number by 1

**Usage:** `version patch set down`



## `version patch reset`

Reset the subversions

**Usage:** `version patch reset`



## `version alpha`

Change the alpha identifier

**Usage:** `version alpha <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier
* `reset` — Reset the subversions



## `version alpha get`

Print the current version

**Usage:** `version alpha get`



## `version alpha set`

Set the version number

**Usage:** `version alpha set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version alpha set +`

Increment the version number by 1

**Usage:** `version alpha set +`



## `version alpha set -`

Decrement the version number by 1

**Usage:** `version alpha set -`



## `version alpha set up`

Increment the version number by 1

**Usage:** `version alpha set up`



## `version alpha set down`

Decrement the version number by 1

**Usage:** `version alpha set down`



## `version alpha rm`

Remove the version identifier

**Usage:** `version alpha rm`



## `version alpha reset`

Reset the subversions

**Usage:** `version alpha reset`



## `version beta`

Change the beta identifier

**Usage:** `version beta <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier
* `reset` — Reset the subversions



## `version beta get`

Print the current version

**Usage:** `version beta get`



## `version beta set`

Set the version number

**Usage:** `version beta set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version beta set +`

Increment the version number by 1

**Usage:** `version beta set +`



## `version beta set -`

Decrement the version number by 1

**Usage:** `version beta set -`



## `version beta set up`

Increment the version number by 1

**Usage:** `version beta set up`



## `version beta set down`

Decrement the version number by 1

**Usage:** `version beta set down`



## `version beta rm`

Remove the version identifier

**Usage:** `version beta rm`



## `version beta reset`

Reset the subversions

**Usage:** `version beta reset`



## `version rc`

Change the release candidate identifier

**Usage:** `version rc <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier
* `reset` — Reset the subversions



## `version rc get`

Print the current version

**Usage:** `version rc get`



## `version rc set`

Set the version number

**Usage:** `version rc set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version rc set +`

Increment the version number by 1

**Usage:** `version rc set +`



## `version rc set -`

Decrement the version number by 1

**Usage:** `version rc set -`



## `version rc set up`

Increment the version number by 1

**Usage:** `version rc set up`



## `version rc set down`

Decrement the version number by 1

**Usage:** `version rc set down`



## `version rc rm`

Remove the version identifier

**Usage:** `version rc rm`



## `version rc reset`

Reset the subversions

**Usage:** `version rc reset`



## `version build`

Change the build identifier

**Usage:** `version build <COMMAND>`

###### **Subcommands:**

* `get` — 
* `set` — Set the build version
* `rm` — 



## `version build get`

**Usage:** `version build get`



## `version build set`

Set the build version

**Usage:** `version build set <VALUE>`

###### **Arguments:**

* `<VALUE>`



## `version build rm`

**Usage:** `version build rm`



## `version get`

Get the current version number as a full SemVer string

**Usage:** `version get`



## `version set`

Set the version number to a specific version

**Usage:** `version set <VAL>`

###### **Arguments:**

* `<VAL>`



## `version version`

Get just the version number as a string with no revision or build identifiers

**Usage:** `version version`



## `version revision`

Get just the revision number as a string with no build identifiers

**Usage:** `version revision`



## `version file`

Track and update the version number in a file

**Usage:** `version file <COMMAND>`

###### **Subcommands:**

* `track` — Add a file to add the version number
* `rm` — Remove a file from tracking the version number
* `update` — Set the version number from a file
* `update-all` — Update all files
* `list` — List tracked files



## `version file track`

Add a file to add the version number

**Usage:** `version file track <PATH> <EXPR>`

###### **Arguments:**

* `<PATH>` — The path to the file to track
* `<EXPR>` — The expression to match the version number

   This expression should be a regex with a single capture group that matches the version number



## `version file rm`

Remove a file from tracking the version number

**Usage:** `version file rm <PATH>`

###### **Arguments:**

* `<PATH>` — The path to the file



## `version file update`

Set the version number from a file

**Usage:** `version file update <PATH>`

###### **Arguments:**

* `<PATH>` — The path to the file



## `version file update-all`

Update all files

**Usage:** `version file update-all`



## `version file list`

List tracked files

**Usage:** `version file list`



## `version package`

Track and update the version number in a file

**Usage:** `version package [PACKAGE_NAME] <COMMAND>`

###### **Subcommands:**

* `major` — Change the major version number
* `minor` — Change the minor version number
* `patch` — Change the patch version number
* `alpha` — Change the alpha identifier
* `beta` — Change the beta identifier
* `rc` — Change the release candidate identifier
* `build` — Change the build identifier
* `get` — Get the current version number as a full SemVer string
* `set` — Set the version number to a specific version
* `version` — Get just the version number as a string with no revision or build identifiers
* `revision` — Get just the revision number as a string with no build identifiers
* `file` — Track and update the version number in a file
* `rm` — Remove a package
* `list` — List tracked packages

###### **Arguments:**

* `<PACKAGE_NAME>` — The package name to track



## `version package major`

Change the major version number

**Usage:** `version package major <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `reset` — Reset the subversions



## `version package major get`

Print the current version

**Usage:** `version package major get`



## `version package major set`

Set the version number

**Usage:** `version package major set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version package major set +`

Increment the version number by 1

**Usage:** `version package major set +`



## `version package major set -`

Decrement the version number by 1

**Usage:** `version package major set -`



## `version package major set up`

Increment the version number by 1

**Usage:** `version package major set up`



## `version package major set down`

Decrement the version number by 1

**Usage:** `version package major set down`



## `version package major reset`

Reset the subversions

**Usage:** `version package major reset`



## `version package minor`

Change the minor version number

**Usage:** `version package minor <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `reset` — Reset the subversions



## `version package minor get`

Print the current version

**Usage:** `version package minor get`



## `version package minor set`

Set the version number

**Usage:** `version package minor set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version package minor set +`

Increment the version number by 1

**Usage:** `version package minor set +`



## `version package minor set -`

Decrement the version number by 1

**Usage:** `version package minor set -`



## `version package minor set up`

Increment the version number by 1

**Usage:** `version package minor set up`



## `version package minor set down`

Decrement the version number by 1

**Usage:** `version package minor set down`



## `version package minor reset`

Reset the subversions

**Usage:** `version package minor reset`



## `version package patch`

Change the patch version number

**Usage:** `version package patch <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `reset` — Reset the subversions



## `version package patch get`

Print the current version

**Usage:** `version package patch get`



## `version package patch set`

Set the version number

**Usage:** `version package patch set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version package patch set +`

Increment the version number by 1

**Usage:** `version package patch set +`



## `version package patch set -`

Decrement the version number by 1

**Usage:** `version package patch set -`



## `version package patch set up`

Increment the version number by 1

**Usage:** `version package patch set up`



## `version package patch set down`

Decrement the version number by 1

**Usage:** `version package patch set down`



## `version package patch reset`

Reset the subversions

**Usage:** `version package patch reset`



## `version package alpha`

Change the alpha identifier

**Usage:** `version package alpha <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier
* `reset` — Reset the subversions



## `version package alpha get`

Print the current version

**Usage:** `version package alpha get`



## `version package alpha set`

Set the version number

**Usage:** `version package alpha set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version package alpha set +`

Increment the version number by 1

**Usage:** `version package alpha set +`



## `version package alpha set -`

Decrement the version number by 1

**Usage:** `version package alpha set -`



## `version package alpha set up`

Increment the version number by 1

**Usage:** `version package alpha set up`



## `version package alpha set down`

Decrement the version number by 1

**Usage:** `version package alpha set down`



## `version package alpha rm`

Remove the version identifier

**Usage:** `version package alpha rm`



## `version package alpha reset`

Reset the subversions

**Usage:** `version package alpha reset`



## `version package beta`

Change the beta identifier

**Usage:** `version package beta <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier
* `reset` — Reset the subversions



## `version package beta get`

Print the current version

**Usage:** `version package beta get`



## `version package beta set`

Set the version number

**Usage:** `version package beta set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version package beta set +`

Increment the version number by 1

**Usage:** `version package beta set +`



## `version package beta set -`

Decrement the version number by 1

**Usage:** `version package beta set -`



## `version package beta set up`

Increment the version number by 1

**Usage:** `version package beta set up`



## `version package beta set down`

Decrement the version number by 1

**Usage:** `version package beta set down`



## `version package beta rm`

Remove the version identifier

**Usage:** `version package beta rm`



## `version package beta reset`

Reset the subversions

**Usage:** `version package beta reset`



## `version package rc`

Change the release candidate identifier

**Usage:** `version package rc <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier
* `reset` — Reset the subversions



## `version package rc get`

Print the current version

**Usage:** `version package rc get`



## `version package rc set`

Set the version number

**Usage:** `version package rc set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1
* `up` — Increment the version number by 1
* `down` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version package rc set +`

Increment the version number by 1

**Usage:** `version package rc set +`



## `version package rc set -`

Decrement the version number by 1

**Usage:** `version package rc set -`



## `version package rc set up`

Increment the version number by 1

**Usage:** `version package rc set up`



## `version package rc set down`

Decrement the version number by 1

**Usage:** `version package rc set down`



## `version package rc rm`

Remove the version identifier

**Usage:** `version package rc rm`



## `version package rc reset`

Reset the subversions

**Usage:** `version package rc reset`



## `version package build`

Change the build identifier

**Usage:** `version package build <COMMAND>`

###### **Subcommands:**

* `get` — 
* `set` — Set the build version
* `rm` — 



## `version package build get`

**Usage:** `version package build get`



## `version package build set`

Set the build version

**Usage:** `version package build set <VALUE>`

###### **Arguments:**

* `<VALUE>`



## `version package build rm`

**Usage:** `version package build rm`



## `version package get`

Get the current version number as a full SemVer string

**Usage:** `version package get`



## `version package set`

Set the version number to a specific version

**Usage:** `version package set <VAL>`

###### **Arguments:**

* `<VAL>`



## `version package version`

Get just the version number as a string with no revision or build identifiers

**Usage:** `version package version`



## `version package revision`

Get just the revision number as a string with no build identifiers

**Usage:** `version package revision`



## `version package file`

Track and update the version number in a file

**Usage:** `version package file <COMMAND>`

###### **Subcommands:**

* `track` — Add a file to add the version number
* `rm` — Remove a file from tracking the version number
* `update` — Set the version number from a file
* `update-all` — Update all files
* `list` — List tracked files



## `version package file track`

Add a file to add the version number

**Usage:** `version package file track <PATH> <EXPR>`

###### **Arguments:**

* `<PATH>` — The path to the file to track
* `<EXPR>` — The expression to match the version number

   This expression should be a regex with a single capture group that matches the version number



## `version package file rm`

Remove a file from tracking the version number

**Usage:** `version package file rm <PATH>`

###### **Arguments:**

* `<PATH>` — The path to the file



## `version package file update`

Set the version number from a file

**Usage:** `version package file update <PATH>`

###### **Arguments:**

* `<PATH>` — The path to the file



## `version package file update-all`

Update all files

**Usage:** `version package file update-all`



## `version package file list`

List tracked files

**Usage:** `version package file list`



## `version package rm`

Remove a package

**Usage:** `version package rm`



## `version package list`

List tracked packages

**Usage:** `version package list`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
