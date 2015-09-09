package t::Helper;
use strict;
use warnings;
use File::Basename;
use File::Spec;
use File::Path ();
use Test::More;

$ENV{TIMETRACKER_HOME} ||= File::Spec->catdir('t', '.TimeTracker-' . basename($0));
File::Path::remove_tree($ENV{TIMETRACKER_HOME}) if -d $ENV{TIMETRACKER_HOME};

sub tt {
  my $path = File::Spec->catfile(qw( script tt ));
  plan skip_all => "Cannot find $path" unless -f $path;
  my $script = do $path || die $@;
  my $class = ref $script;
  no strict 'refs';
  no warnings 'redefine';
  *{"$class\::_diag"} = sub { shift; push @main::diag, @_ };
  *{"$class\::_say"}  = sub { shift; push @main::say,  @_ };
  return $script;
}

sub import {
  my $class  = shift;
  my $caller = caller;

  @main::diag = ();
  @main::say  = ();
  strict->import;
  warnings->import;
  eval "package $caller;use Test::More;1" or die $@;
}

1;
