= For each version bump, do the following =
* Update version in the following workspace items, which should always be in-sync:
    * frontend
    * game-master
    * db-interface
    * shared-lib
* Run frontend benchmarks and statistics scripts
    * See [here](/paddlers-frontend/benchmarks/README.md) for details
    * Then also regenerate the pdf graphs
* Write a changelog for website
* Add release tag to the commit