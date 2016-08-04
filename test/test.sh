#!/bin/bash

# rsync man page
# http://ss64.com/bash/rsync_options.html

# Very useful website
# http://www.jveweb.net/en/archives/2010/11/synchronizing-folders-with-rsync.html

# Configure this file for each directory you want to watch
# If there is an issue with file mismatches, run this file directly to clean up

SOURCE=~/dev/mover/test/source/
TARGET=~/dev/mover/test/target/

# Flag explaination
# -a => 'achive' mode that recurses and preserves nearly all file attributes
# -t => preseve file last changed timestamps
# -u => use delta copy to only copy changed files
# -c => use a file hash instead of the timestamp to determine if changed (useful when changing git branches)
# -v => verbose mode (should be ommitted for normal use)
rsync -atuc --delete $SOURCE $TARGET

# rsync -atz -v --delete ~/dev/mover/test/source/ ~/dev/mover/test/target/
