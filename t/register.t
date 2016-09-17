use t::Helper;
use File::Spec::Functions 'catfile';

my $tt   = t::Helper->tt;
my $year = 1900 + (localtime)[5];
my @args = ("$year-09-17T09:00:00", "17:00:00");

is $tt->cmd_register(@args), 0, 'register with missing args';
like $main::say, qr{import data from other sources}, 'register help';

push @args, 'project-name', 'some description', 'foo,bar';
is $tt->cmd_register(@args), 0, 'register project-name';

@args[0 .. 2] = ("$year-10-17T09:00:00", '17:00:00', 'other');
is $tt->cmd_register(@args), 0, 'register with hh::mm::ss';

@args[1] = '18:00:00';
is $tt->cmd_register(@args), 1, 'register with same time';

$main::diag = $main::say = '';
is $tt->cmd_log('year'), 0, 'cmd_log';
like $main::say, qr{\s+17\s+09:00\s+8:00\s+project-name\s+foo,bar}i, 'log sep';
like $main::say, qr{\s+17\s+09:00\s+8:00\s+other\s+foo,bar}i,        'log oct';

unlink catfile split '/'
  for (
  "t/.TimeTracker-register.t/$year/09/20160917-090000_project_name.trc",
  "t/.TimeTracker-register.t/$year/10/20161017-090000_other.trc",
  );

done_testing;
