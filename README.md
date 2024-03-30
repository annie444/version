# Command-Line Help for `version`

This document contains the help content for the `version` command-line program.

**Command Overview:**

* [`version`↴](#version)
* [`version major`↴](#version-major)
* [`version major get`↴](#version-major-get)
* [`version major set`↴](#version-major-set)
* [`version major set +`↴](#version-major-set-+)
* [`version major set -`↴](#version-major-set--)
* [`version major reset`↴](#version-major-reset)
* [`version minor`↴](#version-minor)
* [`version minor get`↴](#version-minor-get)
* [`version minor set`↴](#version-minor-set)
* [`version minor set +`↴](#version-minor-set-+)
* [`version minor set -`↴](#version-minor-set--)
* [`version minor reset`↴](#version-minor-reset)
* [`version patch`↴](#version-patch)
* [`version patch get`↴](#version-patch-get)
* [`version patch set`↴](#version-patch-set)
* [`version patch set +`↴](#version-patch-set-+)
* [`version patch set -`↴](#version-patch-set--)
* [`version patch reset`↴](#version-patch-reset)
* [`version alpha`↴](#version-alpha)
* [`version alpha get`↴](#version-alpha-get)
* [`version alpha set`↴](#version-alpha-set)
* [`version alpha set +`↴](#version-alpha-set-+)
* [`version alpha set -`↴](#version-alpha-set--)
* [`version alpha rm`↴](#version-alpha-rm)
* [`version beta`↴](#version-beta)
* [`version beta get`↴](#version-beta-get)
* [`version beta set`↴](#version-beta-set)
* [`version beta set +`↴](#version-beta-set-+)
* [`version beta set -`↴](#version-beta-set--)
* [`version beta rm`↴](#version-beta-rm)
* [`version rc`↴](#version-rc)
* [`version rc get`↴](#version-rc-get)
* [`version rc set`↴](#version-rc-set)
* [`version rc set +`↴](#version-rc-set-+)
* [`version rc set -`↴](#version-rc-set--)
* [`version rc rm`↴](#version-rc-rm)
* [`version build`↴](#version-build)
* [`version build get`↴](#version-build-get)
* [`version build set`↴](#version-build-set)
* [`version build rm`↴](#version-build-rm)
* [`version get`↴](#version-get)

## `version`

A tool for managing the version of a project

**Usage:** `version [GENERATOR] [COMMAND]`

###### **Subcommands:**

* `major` — Change the major version number
* `minor` — Change the minor version number
* `patch` — Change the patch version number
* `alpha` — Change the alpha identifier
* `beta` — Change the beta identifier
* `rc` — Change the release candidate identifier
* `build` — Change the build identifier
* `get` — Get the current version number as a full SemVer string

###### **Arguments:**

* `<GENERATOR>` — Generate shell completions

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

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version major set +`

Increment the version number by 1

**Usage:** `version major set +`



## `version major set -`

Decrement the version number by 1

**Usage:** `version major set -`



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

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version minor set +`

Increment the version number by 1

**Usage:** `version minor set +`



## `version minor set -`

Decrement the version number by 1

**Usage:** `version minor set -`



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

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version patch set +`

Increment the version number by 1

**Usage:** `version patch set +`



## `version patch set -`

Decrement the version number by 1

**Usage:** `version patch set -`



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



## `version alpha get`

Print the current version

**Usage:** `version alpha get`



## `version alpha set`

Set the version number

**Usage:** `version alpha set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version alpha set +`

Increment the version number by 1

**Usage:** `version alpha set +`



## `version alpha set -`

Decrement the version number by 1

**Usage:** `version alpha set -`



## `version alpha rm`

Remove the version identifier

**Usage:** `version alpha rm`



## `version beta`

Change the beta identifier

**Usage:** `version beta <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier



## `version beta get`

Print the current version

**Usage:** `version beta get`



## `version beta set`

Set the version number

**Usage:** `version beta set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version beta set +`

Increment the version number by 1

**Usage:** `version beta set +`



## `version beta set -`

Decrement the version number by 1

**Usage:** `version beta set -`



## `version beta rm`

Remove the version identifier

**Usage:** `version beta rm`



## `version rc`

Change the release candidate identifier

**Usage:** `version rc <COMMAND>`

###### **Subcommands:**

* `get` — Print the current version
* `set` — Set the version number
* `rm` — Remove the version identifier



## `version rc get`

Print the current version

**Usage:** `version rc get`



## `version rc set`

Set the version number

**Usage:** `version rc set [VALUE] [COMMAND]`

###### **Subcommands:**

* `+` — Increment the version number by 1
* `-` — Decrement the version number by 1

###### **Arguments:**

* `<VALUE>` — The value to set the version number to



## `version rc set +`

Increment the version number by 1

**Usage:** `version rc set +`



## `version rc set -`

Decrement the version number by 1

**Usage:** `version rc set -`



## `version rc rm`

Remove the version identifier

**Usage:** `version rc rm`



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



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
