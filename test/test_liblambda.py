# -*- coding: utf-8 -*-
# Copyright (c) 2017 Christopher MacGown
#
# Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
# http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
# http://opensource.org/licenses/MIT>, at your option. This file may not be
# copied, modified, or distributed except according to those terms.
#
#
# pylint: disable=missing-docstring

import contextlib
import fcntl
import os
import sys
import re
import time
import unittest

ERROR = None

try:
    import liblambda
except ImportError as err:
    ERROR = err
    liblambda = None


@contextlib.contextmanager
def capture_stdout():
    '''capture_stdout()

       Store the stdout file descriptor and redirect it to the write-side of
       the created pipe. We put the read-side into non-blocking mode so that
       we can capture the output later.

       This is only necessary because Rust writes directly to the stdout fd
       and doing the typical sys.stdout song and dance in Python doesn't
       capture things coming out of `liblambda`.

       This is a context-manager and it yields the read-side of the pipe, so
       it needs to be read and closed (for good housekeeping).
    '''

    # Setup pipe
    read, write = os.pipe()
    read, write = os.fdopen(read, 'rb'), os.fdopen(write, 'wb')

    fcntl.fcntl(read, fcntl.F_SETFL, os.O_NONBLOCK)

    fd = sys.stdout.fileno() # pylint: disable=invalid-name
    with os.fdopen(os.dup(fd), 'wb') as copied:
        sys.stdout.flush()
        os.dup2(write.fileno(), fd) # redirect STDOUT -> the write FD.

        try:
            yield read
        finally:
            sys.stdout.flush()
            os.dup2(copied.fileno(), fd) # redirect back.
            write.close()


def now_in_millis():
    return int(round(time.time() * 1000.0))


class FakeContext(object):  # pylint: disable=too-few-public-methods,too-many-instance-attributes
    def __init__(self, timeout=3000):
        self._start = now_in_millis()
        self._timeout = timeout

        self.function_name = 'fake_function'
        self.function_version = '$LATEST'
        self.invoked_function_arn = 'arn:aws:lambda:XX-TEST-1:999999999999:function:fake_function' # pylint: disable=line-too-long
        self.memory_limit_in_mb = '128'
        self.aws_request_id = '1f8958d8-b20b-4a3c-b8fb-78896d10a9e5'
        self.log_group_name = '/aws/lambda/fake_function'
        # Date and coordinates of the Battle of Hastings - just for test data.
        self.log_stream_name = '1066/10/14/[$LATEST]00000000000000000000505443002915' # pylint: disable=line-too-long

    def get_remaining_time_in_millis(self):
        return self._timeout - (now_in_millis() - self._start)


def consume(reader):
    try:
        return reader.read().decode(sys.stdout.encoding)
    finally:
        reader.close()

class TestCrowbar(unittest.TestCase):
    def setUp(self):
        self.context = FakeContext(timeout=100)

        if not getattr(self, 'assertRegex', None):
            # assertRegexpMatches is deprecated in 3.6, so make sure python2.7
            # calls the method the same thing.
            setattr(self, 'assertRegex', self.assertRegexpMatches)

    def test_00_import_liblambda(self): # pylint: disable=no-self-use
        # This makes the import failure a little friendlier.
        if liblambda is None:
            print("Could not import liblambda: {}".format(ERROR))

    @unittest.skipIf(liblambda is None, "Could not import liblambda")
    @unittest.skipUnless(os.environ["EXAMPLE"] == "echo", "DISABLED")
    def test_01_echo_short_timeout(self):
        expectation = re.compile(r'hello cloudwatch logs from (?P<name>\w+) '
                                 r'version (?P<version>\${0,1}\w+),'
                                 r'.(?P<time>\d+) ms.*')

        time.sleep(0.01)

        with capture_stdout() as stdout:
            self.assertEqual(liblambda.handler("echo", self.context), "echo")

        output = consume(stdout)
        matches = re.match(expectation, output)

        self.assertRegex(output, expectation, "")
        self.assertIn(int(matches.group('time')),
                      [87, 88, 89, 90],
                      "{} not in [87, 88, 89, 90]".format(matches.group('time')))

    @unittest.skipIf(liblambda is None, "Could not import liblambda")
    @unittest.skipUnless(os.environ["EXAMPLE"] == "ec2_regions", "DISABLED")
    def test_01_ec2_regions_short_timeout(self):  # pylint: disable=invalid-name
        os.environ["AWS_DEFAULT_REGION"] = "us-east-1"

        with capture_stdout() as stdout:
            self.assertEqual(liblambda.handler("list-regions", self.context),
                             ['ap-south-1',
                              'eu-west-2',
                              'eu-west-1',
                              'ap-northeast-2',
                              'ap-northeast-1',
                              'sa-east-1',
                              'ca-central-1',
                              'ap-southeast-1',
                              'ap-southeast-2',
                              'eu-central-1',
                              'us-east-1',
                              'us-east-2',
                              'us-west-1',
                              'us-west-2',])
        output = consume(stdout)
        self.assertEqual(output, "", "Unexpected STDOUT output")

    @unittest.skipIf(liblambda is None, "Could not import liblambda")
    @unittest.skipUnless(os.environ["EXAMPLE"] == "echo", "DISABLED")
    def test_02_echo_long_timeout(self):
        # This test is a duplicate of test_01_echo, but with a longer deadline. Not necessarily
        # the most exhaustive method of testing, but I wanted to show that.
        expectation = re.compile(r'hello cloudwatch logs from (?P<name>\w+) '
                                 r'version (?P<version>\${0,1}\w+),'
                                 r'.(?P<time>\d+) ms.*')

        context = FakeContext()  # 3 seconds.
        time.sleep(0.001)
        with capture_stdout() as stdout:
            self.assertEqual(liblambda.handler("echo", context), "echo")

        output = consume(stdout)
        matches = re.match(expectation, output)

        self.assertRegex(output, expectation, "unexpected")
        self.assertIn(int(matches.group('time')),
                      [2998, 2999, 3000],
                      "{} not in [2998, 2999, 3000]".format(matches.group('time')))

    @unittest.skipIf(liblambda is None, "Could not import liblambda")
    @unittest.skipUnless(os.environ["EXAMPLE"] == "ec2_regions", "DISABLED")
    def test_02_ec2_regions_long_timeout(self):  # pylint: disable=invalid-name
        context = FakeContext()  # 3 seconds.
        time.sleep(0.001)
        with capture_stdout() as stdout:
            self.assertEqual(liblambda.handler("list-regions", context),
                             ['ap-south-1',
                              'eu-west-2',
                              'eu-west-1',
                              'ap-northeast-2',
                              'ap-northeast-1',
                              'sa-east-1',
                              'ca-central-1',
                              'ap-southeast-1',
                              'ap-southeast-2',
                              'eu-central-1',
                              'us-east-1',
                              'us-east-2',
                              'us-west-1',
                              'us-west-2',])
        output = consume(stdout)
        self.assertEqual(output, "", "Unexpected STDOUT output")

if __name__ == '__main__':

    unittest.main()
