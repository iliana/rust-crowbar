# crowbar tests

In order to test the ec2-regions example, you'll need to export your AWS credentials as environment
variables or create a credentials file at `~/.aws/credentials` formatted like so:


    [default]
    aws_access_key_id=<YOUR AWS ACCESS ID>
    aws_secret_access_key=<YOUR AWS ACCESS KEY SECRET>


The easiest way to test is to use the `test-all-local` or `test-all-docker` Makefile targets. However,
you can use any of the below to test one or more of the examples locally or built with the builder.


    make echo test-local        # Locally build and test the 'echo' example.
    make ec2-regions test-local # Locally build and test the 'ec2-regions' example.
    make test-all-local         # Locally build and test both examples.

    make build-echo test-docker        # Build and test the 'echo' example against amazonlinux:latest
    make build-ec2-regions test-docker # Build and test the 'ec2-regions' example against amazonlinux:latest
    make test-all-docker	       # Build and test both examples against amazonlinux:latest
