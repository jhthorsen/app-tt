use t::Helper;

my $tt = t::Helper->tt;

is $tt->cmd_stop, 3, 'no previous event';

#is $tt->cmd_start, 0, 'start';

done_testing;
