= For each version bump, do the following =
* Update version in the following workspace items, which should always be in-sync:
    * frontend
    * game-master
    * db-interface
    * shared-lib
* Run frontend benchmarks and statistics scripts
    * See [here](/paddlers-frontend/benchmarks/README.md) for details
    * Then also regenerate the pdf graphs
* Make sure release builds locally without errors
* Commit, add release tag to the commit (docker hub can now start running)
* Write a changelog for website