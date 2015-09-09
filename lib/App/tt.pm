package App::tt;

=head1 NAME

App::tt - Time tracking application

=head1 VERSION

0.01

=head1 DESCRIPTION

L<App::tt> is an application that can track how much time you spend on an
project from command line.

It is inspired by L<App::TimeTracker> and share the same log file format,
but it has (in my humble opinion) a simpler interface and easier to install.

=head1 SYNOPSIS

=head2 Basic usage

  # Start to track time
  $ cd $HOME/git/my-project
  $ tt start
  # Work, work, work, cd ..., do other stuff
  $ tt stop

=head2 A bit more complex

  # Start to work on an event and add a tag
  $ tt start -t ISSUE-999 -p some-project-at-work
  # Add another tag to the same event and add a --comment
  $ tt stop -t GITHUB-1005 "Today I was mostly in meetings"

=head2 Available actions

  $ tt {analyze,log,start,stop,status,register}
  $ tt start -h

Each action takes "-h" for more details.

=cut

use strict;
use warnings;

our $VERSION = '0.01';

=head1 COPYRIGHT AND LICENSE

Copyright (C) 2014, Jan Henning Thorsen

This program is free software, you can redistribute it and/or modify it under
the terms of the Artistic License version 2.0.

=head1 AUTHOR

Jan Henning Thorsen - C<jhthorsen@cpan.org>

=cut

1;
