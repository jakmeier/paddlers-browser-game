= For each version bump, do the following =
* Run frontend benchmarks and statistics scripts
    * See [here](/paddlers-frontend/benchmarks/README.md) for details
    * Then also regenerate the pdf graphs
* Write a changelog for website
* Make sure release builds locally without errors
* Add release tag to the commit
* Update version in the following workspace items, which should always be in-sync:
    * frontend
    * game-master
    * db-interface
    * shared-lib