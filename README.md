Minimal directory creation bash builtin.

Documentation for [std::fs::create_dir_all](https://doc.rust-lang.org/std/fs/fn.create_dir_all.html)
states that it should not an error while trying to create the same
directory concurrently:

> Notable exception is made for situations where any of the directories
> specified in the path could not be created as it was being created
> concurrently. Such cases are considered to be successful. That is,
> calling create_dir_all concurrently from multiple threads or processes
> is guaranteed not to fail due to a race condition with itself.


# Usage

Createdir will try to create each directory that you pass as
arguments:

`createdir /tmp/dir/to/create /tmp/an/another/dir/to/create`

It may fail for a permission denied for instance in that case
the remaining directories are not created.

On success it does not print anything. It is not an error if
the directory to be created exists already.

