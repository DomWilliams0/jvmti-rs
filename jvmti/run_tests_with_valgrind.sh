#!/bin/bash

suppressions=/tmp/valgrind-suppressions
marker=jvmti-rs-valgrind-error-crap
results_dir=test_results
cat <<EOF >$suppressions
{
   naughty_jvm
   Memcheck:Addr4
   ...
   obj:/usr/lib/jvm/*
   ...
}
{
   naughty_jvm
   Memcheck:Addr4
   ...
   obj:???
}
EOF

# $1: test name
function do_test() {
	output="$results_dir/$name.out"
	echo running test $path
	cargo with "valgrind --suppressions=$suppressions --error-markers=$marker --log-file=$output" -- test --test $name

	if [ $? -ne 0 ] || grep -q $marker $output; then
		echo test \"$name\" failed
		exit 1
	fi
}

rm -rf --one-file-system $results_dir
mkdir -p $results_dir
for path in tests/*; do
	name=$(basename $path .rs)
	if [[ $name = "common" ]]; then continue; fi
	do_test $name &
done
echo all done
