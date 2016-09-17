use t::Helper;
use File::Path 'remove_tree';
use File::Spec::Functions 'catdir';

plan skip_all => '.git missing' unless -d '.git';

remove_tree(catdir qw(t .TimeTracker-start-stop.t));

my $tt = t::Helper->tt;

is $tt->cmd_stop, 3, 'no previous event';

# no data
$main::out = '';
is $tt->cmd_status, 3, 'cmd_status';
like $main::out, qr{No event is being tracked}, 'no event';

$main::out = '';
is $tt->cmd_log('year'), 0, 'cmd_log';
like $main::out, qr{0h 0m 0s}s, 'empty log';

# stop at same day
$main::out = '';
$tt->{now} = Time::Piece->new(time - (time % 86400) - 140000);
is $tt->cmd_start('yesterday'), 0, 'cmd_start yesterday';
reset_tt();
is $tt->cmd_stop('17:00'), 0, 'cmd_stop yesterday';

$main::out = '';
is $tt->cmd_log('year'), 0, 'cmd_log';
like $main::out, qr{5h 53m 20s}i, 'total work time [1]';

# cancel start because of TIMETRACKER_MIN_TIME
reset_tt();
is $tt->cmd_start('cancel'), 0, 'cmd_start';
$main::out = '';
is $tt->cmd_stop, 52, 'cancel start';

# regular start+stop
reset_tt();
is $tt->cmd_start('regular', '00:02'), 0, 'cmd_start';
is $tt->cmd_status, 0, 'cmd_status';
like $main::out, qr{Been working on}, 'been working';
is $tt->cmd_stop('00:09'), 0, 'cmd_stop';

reset_tt();
$main::out = '';
$tt->cmd_log('year');
like $main::out, qr{6h 0m 20s}i, 'total work time [2]';

done_testing;

sub reset_tt {
  delete $tt->{$_} for qw(custom_now now project);
  $tt->{tag} = [];
}
